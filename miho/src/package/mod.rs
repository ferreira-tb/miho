pub mod manifest;

use crate::agent::Agent;
use crate::dependency::{DependencyKind, DependencyTree};
use crate::prelude::*;
use crate::release::Release;
use crate::version::VersionExt;
use crate::{command, return_if_ne};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use manifest::{ManifestBox, ManifestKind};
use semver::{Op, Version};
use serde_json::Value;
use std::cmp::Ordering;
use std::fmt;
use std::path::{Path, PathBuf};

pub struct Package {
  pub name: String,
  pub version: Version,
  pub path: PathBuf,
  manifest: ManifestBox,
}

impl Package {
  /// Creates a representation of the package based on the manifest at `path`.
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
    let path = path.as_ref();
    let kind = ManifestKind::try_from(path)?;
    let manifest = kind.read(path)?;

    let package = Self {
      name: manifest.name().to_owned(),
      version: manifest.version()?,
      path: path.to_path_buf(),
      manifest,
    };

    Ok(package)
  }

  pub fn search<P, S>(path: &[P], only: Option<&[S]>) -> Result<Vec<Self>>
  where
    P: AsRef<Path> + fmt::Debug,
    S: AsRef<str>,
  {
    info!("searching packages in: {path:?}");
    let Some((first, other)) = path.split_first() else {
      return Ok(Vec::new());
    };

    let mut walker = WalkBuilder::new(first);

    for path in other {
      walker.add(path);
    }

    let glob = build_globset()?;
    let mut packages = Vec::new();

    for entry in walker.build().flatten() {
      if is_match(&glob, &entry) {
        let path = entry.path().canonicalize()?;
        let package = Package::new(path);
        if matches!(package, Ok(ref it) if !packages.contains(it)) {
          let package = package.unwrap();
          info!("found: {:?}", package.path.display());
          packages.push(package);
        }
      }
    }

    if matches!(only, Some(it) if !it.is_empty()) {
      let only = only
        .unwrap_or_default()
        .iter()
        .map(AsRef::as_ref)
        .collect_vec();

      info!("filtering: {only:?}");
      packages.retain(|it| only.contains(&it.name.as_str()));
    }

    if packages.is_empty() {
      bail!("no valid package found");
    }

    packages.sort_unstable();

    Ok(packages)
  }

  pub fn agent(&self) -> Agent {
    self.manifest.agent()
  }

  pub fn bump(self, release: &Release) -> Result<()> {
    let version = self.version.with_release(release);
    self.manifest.bump(&self, version)
  }

  pub fn update(self, tree: DependencyTree, release: &Option<Release>) -> Result<()> {
    let targets = tree
      .dependencies
      .into_iter()
      .filter_map(|it| it.into_target(release))
      .collect_vec();

    self.manifest.update(&self, &targets)
  }
}

impl fmt::Debug for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Package")
      .field("name", &self.name)
      .field("version", &self.version)
      .field("path", &self.path)
      .field("manifest", &self.manifest.name())
      .finish()
  }
}

impl PartialEq for Package {
  fn eq(&self, other: &Self) -> bool {
    self.path == other.path
  }
}

impl Eq for Package {}

impl PartialOrd for Package {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Package {
  fn cmp(&self, other: &Self) -> Ordering {
    return_if_ne!(self.agent().cmp(&other.agent()));
    return_if_ne!(self.name.cmp(&other.name));
    return_if_ne!(self.version.cmp(&other.version));

    self.path.cmp(&other.path)
  }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GlobalPackage {
  pub agent: Agent,
  pub dependencies: Vec<(String, Version)>,
}

impl GlobalPackage {
  // This will eventually be updated to also support Cargo.
  pub async fn get() -> Result<Vec<Self>> {
    let mut packages = Vec::with_capacity(1);

    packages.push(GlobalPackage {
      agent: Agent::Npm,
      dependencies: Self::node_dependencies().await?,
    });

    Ok(packages)
  }

  async fn node_dependencies() -> Result<Vec<(String, Version)>> {
    let output = command!("npm")
      .args(["list", "--global", "--json"])
      .output()
      .await?;

    if !output.status.success() {
      let stderr = String::from_utf8_lossy(&output.stderr);
      bail!(stderr.into_owned());
    }

    let json: Value = serde_json::from_slice(&output.stdout)?;
    trace!(npm_list_output = ?json);

    let mut dependencies = Vec::new();
    if let Some(map) = json
      .get("dependencies")
      .and_then(Value::as_object)
    {
      for (name, value) in map {
        let version = value
          .as_object()
          .and_then(|it| it.get("version"))
          .and_then(Value::as_str)
          .and_then(|it| Version::parse(it).ok());

        if let Some(version) = version {
          dependencies.push((name.clone(), version));
        }
      }
    }

    trace!(node_dependencies = ?dependencies);

    Ok(dependencies)
  }

  pub async fn update(self, tree: DependencyTree, release: &Option<Release>) -> Result<()> {
    let targets = tree
      .dependencies
      .into_iter()
      .filter_map(|it| it.into_target(release))
      .collect_vec();

    for target in targets {
      let arg = format!("{}@{}", target.dependency.name, target.comparator);
      command!("npm")
        .args(["install", &arg, "--global"])
        .spawn()?
        .wait()
        .await?;
    }

    Ok(())
  }
}

pub trait PackageDisplay {
  fn display(&self) -> String;
}

impl PackageDisplay for Package {
  fn display(&self) -> String {
    let agent = self.agent().to_string().bright_magenta().bold();
    let name = self.name.bright_yellow().bold();
    format!("[ {agent} ] {name}")
  }
}

impl PackageDisplay for GlobalPackage {
  fn display(&self) -> String {
    self
      .agent
      .to_string()
      .bright_magenta()
      .bold()
      .to_string()
  }
}

pub trait PackageDependencyTree {
  fn dependency_tree(&self) -> DependencyTree;
}

impl PackageDependencyTree for Package {
  fn dependency_tree(&self) -> DependencyTree {
    self.manifest.dependency_tree()
  }
}

impl PackageDependencyTree for GlobalPackage {
  fn dependency_tree(&self) -> DependencyTree {
    let mut tree = DependencyTree::new(self.agent);
    for (name, version) in &self.dependencies {
      tree.add(
        name,
        version.as_comparator(Op::Caret),
        DependencyKind::Normal,
      );
    }

    tree
  }
}

fn build_globset() -> Result<GlobSet> {
  let mut builder = GlobSetBuilder::new();

  macro_rules! add {
    ($kind:ident) => {
      let glob = ManifestKind::$kind.glob();
      builder.add(Glob::new(glob)?);
    };
  }

  add!(CargoToml);
  add!(PackageJson);
  add!(TauriConfJson);

  builder.build().map_err(Into::into)
}

fn is_match(glob: &GlobSet, entry: &DirEntry) -> bool {
  if !glob.is_match(entry.path()) {
    return false;
  }

  matches!(entry.file_type(), Some(it) if !it.is_dir())
}
