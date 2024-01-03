mod package_type;
mod parser;
mod search;
mod transaction;

use crate::semver::{ReleaseType, Version};
use anyhow::Result;
pub use package_type::PackageType;
pub use parser::PackageParser;
pub use search::SearchBuilder;
use std::fmt;
pub use transaction::{Operation, Transaction};

/// Represents a package of a given type.
pub struct Package {
  pub package_type: PackageType,
  pub name: String,
  pub version: Version,
  pub path: String,
  pub op: Operation,
}

impl Package {
  /// Creates a representation of the package at `path`.
  ///
  /// # Errors
  /// This will return an error if the file in the path provided is not a valid package.
  pub fn new<T: AsRef<str>>(
    path: T,
    release_type: &ReleaseType,
    pre_id: Option<&str>,
  ) -> Result<Self> {
    let path = path.as_ref();
    let package_type = package_type::parse_type(path)?;
    let package = package_type.to_package(path, release_type, pre_id)?;
    Ok(package)
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let filename = self.package_type.filename();
    write!(f, "{} ({})", self.name, filename.to_lowercase())
  }
}
