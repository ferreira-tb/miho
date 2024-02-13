mod kind;

use super::dependency;
use super::{Agent, Package};
pub use kind::Kind;
use semver::Version;
use std::path::Path;

pub(super) type ManifestBox = Box<dyn Handler + Send + Sync>;

trait Manifest {
  type Value;

  fn read<P: AsRef<Path>>(manifest_path: P) -> crate::Result<ManifestBox>;
  fn read_as_value<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Self::Value>;
}

pub trait Handler {
  fn agent(&self) -> Agent;
  fn bump(&self, package: &Package, new_version: Version) -> crate::Result<()>;
  fn filename(&self) -> &str;
  fn name(&self) -> &str;
  fn update_dependencies(&self) -> crate::Result<()>;
  fn version(&self) -> crate::Result<Version>;

  fn dependency_tree(&self) -> dependency::Tree {
    dependency::Tree::new(self.agent())
  }
}
