mod manifest;
mod search;

use crate::versioning::semver::{ReleaseType, Version};
use anyhow::Result;
use manifest::{ManifestHandler, ManifestType};
pub use search::SearchBuilder;
use std::fmt;
use std::path::{Path, PathBuf};

pub struct Package {
  pub name: String,
  pub version: Version,
  pub path: PathBuf,
  manifest: Box<dyn ManifestHandler>,
}

impl Package {
  /// Create a representation of the package from the manifest at `path`.
  pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
    let path = path.as_ref();
    let package_type = ManifestType::try_from(path)?;
    let manifest = package_type.read_source(path)?;

    let package = Self {
      name: manifest.name().to_string(),
      version: manifest.version()?,
      path: path.to_path_buf(),
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
