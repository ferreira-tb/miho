mod agent;
mod manifest_type;

use super::Package;
pub use manifest_type::ManifestType;
use semver::Version;
use serde::Serialize;
// use std::collections::HashMap;
use std::path::Path;

trait Manifest: Serialize + std::fmt::Debug {
  type Value;

  fn read<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Box<dyn ManifestHandler>>;
  fn read_as_value<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Self::Value>;
}

pub trait ManifestHandler {
  /// Bump the package to a specific version.
  fn bump(&self, package: &Package, new_version: Version) -> crate::Result<()>;
  // fn dependencies(&self) -> crate::Result<HashMap<String, Version>>;
  fn filename(&self) -> &str;
  fn name(&self) -> &str;
  fn update(&self) -> crate::Result<()>;
  fn version(&self) -> crate::Result<Version>;
}
