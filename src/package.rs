mod package_type;
mod search;

use crate::versioning::semver::{ReleaseType, Version};
use anyhow::Result;
use package_type::PackageType;
pub use search::SearchBuilder;
use std::fmt;

/// Represents a package of a given type.
pub struct Package {
  pub name: String,
  pub version: Version,
  pub path: String,
  source: Box<dyn PackageHandler>,
}

impl Package {
  /// Creates a representation of the package at `path`.
  ///
  /// # Errors
  /// This will return an error if the file in the path provided is not a valid package.
  pub fn new<P: AsRef<str>>(path: P) -> Result<Self> {
    let path = path.as_ref();
    let package_type = PackageType::try_from(path)?;
    let source = package_type.read_source(path)?;

    let package = Self {
      name: source.name().to_string(),
      version: source.version()?,
      path: path.to_string(),
      source,
    };

    Ok(package)
  }

  pub fn bump(&self, rt: &ReleaseType, pre_id: Option<&str>) -> Result<()> {
    let new_version = self.version.inc(rt, pre_id)?;
    self.source.bump(self, new_version)
  }

  pub fn filename(&self) -> &str {
    self.source.filename()
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let filename = self.filename();
    write!(f, "{} ({})", self.name, filename.to_lowercase())
  }
}

trait PackageHandler {
  fn bump(&self, package: &Package, new_version: Version) -> Result<()>;
  fn filename(&self) -> &str;
  fn name(&self) -> &str;
  fn version(&self) -> Result<Version>;
}
