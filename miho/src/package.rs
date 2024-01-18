mod manifest;
mod search;

use crate::semver::{ReleaseType, Version};
use anyhow::Result;
use manifest::{ManifestHandler, ManifestType};
pub use search::SearchBuilder;
use std::fmt;
use std::path::{Path, PathBuf};

pub struct Package {
  pub name: String,
  pub version: Version,
  pub manifest_path: PathBuf,
  manifest: Box<dyn ManifestHandler>,
}

impl Package {
  /// Create a representation of the package from the manifest at `path`.
  pub fn new<P: AsRef<Path>>(manifest_path: P) -> Result<Self> {
    let manifest_path = manifest_path.as_ref();
    let manifest_type = ManifestType::try_from(manifest_path)?;
    let manifest = manifest_type.read_source(manifest_path)?;

    let package = Self {
      name: manifest.name().to_string(),
      version: manifest.version()?,
      manifest_path: manifest_path.to_path_buf(),
      manifest,
    };

    Ok(package)
  }

  pub fn bump(&self, rt: &ReleaseType, pre_id: Option<&str>) -> Result<()> {
    let new_version = self.version.inc(rt, pre_id)?;
    self.manifest.bump(self, new_version)
  }

  pub fn filename(&self) -> &str {
    self.manifest.filename()
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let filename = self.filename();
    write!(f, "{} ({})", self.name, filename.to_lowercase())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
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
      package.bump(&ReleaseType::Patch, None).unwrap();

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
