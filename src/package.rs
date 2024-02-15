mod agent;
pub mod dependency;
pub mod manifest;

use crate::release::Release;
use crate::return_if_ne;
use crate::version::{Version, VersionExt};
pub use agent::Agent;
use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use std::cmp::Ordering;
use std::fmt;
use std::path::{Path, PathBuf};

pub struct Package {
  pub name: String,
  pub version: Version,
  pub path: PathBuf,
  manifest: manifest::ManifestBox,
}

impl Package {
  /// Creates a representation of the package based on the manifest at `path`.
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
    let path = path.as_ref();
    let kind = manifest::Kind::try_from(path)?;
    let manifest = kind.read(path)?;

    let package = Self {
      name: manifest.name().to_owned(),
      version: manifest.version()?,
      path: path.to_path_buf(),
      manifest,
    };

    Ok(package)
  }

  pub fn search<P: AsRef<Path>>(path: &[P]) -> Result<Vec<Self>> {
    let Some((first, other)) = path.split_first() else {
      return Ok(Vec::default());
    };

    let mut walker = WalkBuilder::new(first);
    walker.add_ignore(".mihoignore");

    for path in other {
      walker.add(path);
    }

    let glob = build_globset();
    let mut packages = Vec::new();

    for entry in walker.build() {
      let entry = entry?;
      if is_match(&glob, &entry) {
        let path = entry.path().canonicalize()?;
        if let Ok(package) = Self::new(path) {
          packages.push(package);
        }
      }
    }

    packages.sort_unstable();

    Ok(packages)
  }

  #[must_use]
  pub fn agent(&self) -> Agent {
    self.manifest.agent()
  }

  pub fn bump(self, release: &Release) -> Result<()> {
    let version = self.version.with_release(release);
    self.manifest.bump(&self, version)
  }

  #[must_use]
  pub fn dependency_tree(&self) -> dependency::Tree {
    self.manifest.dependency_tree()
  }

  #[must_use]
  pub fn filename(&self) -> &str {
    self.manifest.filename()
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} ({})", self.name, self.agent())
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
    return_if_ne!(self.name.cmp(&other.name));
    return_if_ne!(self.version.cmp(&other.version));

    self.path.cmp(&other.path)
  }
}

fn build_globset() -> GlobSet {
  let mut builder = GlobSetBuilder::new();

  macro_rules! add {
    ($kind:ident) => {
      let glob = manifest::Kind::$kind.glob();
      builder.add(Glob::new(glob).expect("hardcoded glob should always be valid"));
    };
  }

  add!(CargoToml);
  add!(PackageJson);
  add!(TauriConfJson);

  builder.build().unwrap()
}

fn is_match(glob: &GlobSet, entry: &DirEntry) -> bool {
  if !glob.is_match(entry.path()) {
    return false;
  }

  matches!(entry.file_type(), Some(t) if t.is_file())
}
