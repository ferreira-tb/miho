mod kind;

use super::dependency;
use super::{Agent, Package};
use crate::version::Version;
use anyhow::Result;
pub use kind::Kind;
use std::path::Path;

pub(super) type ManifestBox = Box<dyn Handler + Send + Sync>;

trait Manifest {
  type Value;

  fn read<P: AsRef<Path>>(path: P) -> Result<ManifestBox>;
  fn read_as_value<P: AsRef<Path>>(path: P) -> Result<Self::Value>;
}

pub trait Handler {
  fn agent(&self) -> Agent;
  fn bump(&self, package: &Package, new_version: Version) -> Result<()>;
  fn filename(&self) -> &str;
  fn name(&self) -> &str;
  fn update(&self, package: &Package, batch: Vec<dependency::Update>) -> Result<()>;
  fn version(&self) -> Result<Version>;

  fn dependency_tree(&self) -> dependency::Tree {
    dependency::Tree::new(self.agent())
  }
}
