mod agent;
pub mod dependency;
pub mod manifest;

use crate::release::Release;
use crate::return_if_ne;
use crate::version::{Version, VersionExt};
pub use agent::Agent;
use anyhow::Result;
use dependency::Tree;
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use std::cmp::Ordering;
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

    for path in other {
      walker.add(path);
    }

    let glob = build_globset();
    let mut packages = Vec::new();

    for entry in walker.build() {
      let entry = entry?;
      if is_match(&glob, &entry) {
        let path = entry.path().canonicalize()?;
        let package = Self::new(path);

        if matches!(package, Ok(ref p) if !packages.contains(p)) {
          packages.push(package.unwrap());
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

  pub fn update(self, tree: Tree, release: &Option<Release>) -> Result<()> {
    let dependencies: Vec<dependency::Target> = tree
      .dependencies
      .into_iter()
      .filter_map(|dep| dep.into_target(release))
      .collect();

    self.manifest.update(&self, dependencies)
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

#[cfg(test)]
mod tests {
  use super::Package;
  use crate::release::Release;
  use crate::version::BuildMetadata;
  use std::path::{Path, PathBuf};
  use std::{env, fs};

  fn find_mocks_dir<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
    let path = path.as_ref();
    if path.is_dir() {
      for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name() == "mocks" && entry.file_type().unwrap().is_dir() {
          return Some(entry.path());
        }
      }

      return find_mocks_dir(path.parent().unwrap());
    }

    None
  }

  #[test]
  fn should_find_package() {
    let entries = Package::search(&["."]).unwrap();
    let cwd = env::current_dir().unwrap();

    let toml = cwd.join("Cargo.toml").canonicalize().unwrap();
    let toml = toml.to_str().unwrap();

    if !entries.iter().any(|p| p.path.to_str().unwrap() == toml) {
      panic!("Cargo.toml not found");
    }
  }

  macro_rules! create_package {
    ($manifest:expr) => {{
      let mocks = find_mocks_dir(env::current_dir().unwrap()).unwrap();
      Package::new(mocks.join($manifest)).unwrap()
    }};
  }

  #[test]
  fn should_create_package_from_cargo_toml() {
    let package = create_package!("Cargo.toml");
    assert_eq!(package.name, "cargo-toml");
    assert_eq!(package.filename(), "Cargo.toml");
  }

  #[test]
  fn should_create_package_from_package_json() {
    let package = create_package!("package.json");
    assert_eq!(package.name, "package-json");
    assert_eq!(package.filename(), "package.json");
  }

  #[test]
  fn should_create_package_from_tauri_conf_json() {
    let package = create_package!("tauri.conf.json");
    assert_eq!(package.name, "tauri-conf-json");
    assert_eq!(package.filename(), "tauri.conf.json");
  }

  macro_rules! bump {
    ($manifest:expr) => {
      let mocks = find_mocks_dir(env::current_dir().unwrap()).unwrap();
      let path = mocks.join($manifest);

      let package = Package::new(&path).unwrap();
      let current_patch = package.version.patch;

      let release = Release::Patch(BuildMetadata::EMPTY);
      package.bump(&release).unwrap();

      let package = Package::new(path).unwrap();
      assert_eq!(package.version.patch, current_patch + 1);
    };
  }

  #[test]
  fn should_bump_cargo_toml() {
    bump!("Cargo.toml");
  }

  #[test]
  fn should_bump_package_json() {
    bump!("package.json");
  }

  #[test]
  fn should_bump_tauri_conf_json() {
    bump!("tauri.conf.json");
  }
}
