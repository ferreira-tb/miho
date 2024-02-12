mod manifest_type;

use super::dependency::DependencyTreeBuilder;
use super::{Agent, Package};
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
  fn agent(&self) -> Agent;
  fn bump(&self, package: &Package, new_version: Version) -> crate::Result<()>;
  fn filename(&self) -> &str;
  fn name(&self) -> &str;
  fn update(&self) -> crate::Result<()>;
  fn version(&self) -> crate::Result<Version>;

  fn dependency_tree(&self) -> DependencyTreeBuilder {
    DependencyTreeBuilder::new(self.agent())
  }
}
