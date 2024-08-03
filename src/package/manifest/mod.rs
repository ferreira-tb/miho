mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::Package;
use crate::agent::Agent;
use crate::dependency::{self, Target};
use crate::prelude::*;
use cargo_toml::CargoToml;
use dependency::DependencyTree;
use globset::Glob;
use package_json::PackageJson;
use semver::Version;
use std::path::Path;
use strum::{EnumIter, IntoEnumIterator};
use tauri_conf_json::TauriConfJson;

pub(super) type ManifestBox = Box<dyn Handler + Send + Sync>;

trait Manifest {
  type Value;

  const FILENAME: &'static str;

  fn read<P: AsRef<Path>>(path: P) -> Result<ManifestBox>;
  fn read_as_value<P: AsRef<Path>>(path: P) -> Result<Self::Value>;
}

pub trait Handler {
  fn agent(&self) -> Agent;
  fn bump(&self, package: &Package, new_version: Version) -> Result<()>;
  fn name(&self) -> &str;
  fn update(&self, package: &Package, targets: &[Target]) -> Result<()>;
  fn version(&self) -> Result<Version>;

  fn dependency_tree(&self) -> DependencyTree {
    DependencyTree::new(self.agent())
  }
}

#[derive(Debug, EnumIter)]
pub enum ManifestKind {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl ManifestKind {
  pub(crate) fn read<P: AsRef<Path>>(&self, path: P) -> Result<ManifestBox> {
    match self {
      ManifestKind::CargoToml => CargoToml::read(path),
      ManifestKind::PackageJson => PackageJson::read(path),
      ManifestKind::TauriConfJson => TauriConfJson::read(path),
    }
  }

  pub(crate) fn glob(&self) -> &str {
    match self {
      ManifestKind::CargoToml => "**/Cargo.toml",
      ManifestKind::PackageJson => "**/package.json",
      ManifestKind::TauriConfJson => "**/tauri.conf.json",
    }
  }
}

impl TryFrom<&Path> for ManifestKind {
  type Error = anyhow::Error;

  fn try_from(path: &Path) -> Result<Self> {
    for kind in ManifestKind::iter() {
      let glob = Glob::new(kind.glob())?.compile_matcher();
      if glob.is_match(path) {
        return Ok(kind);
      }
    }

    let path = path.to_string_lossy().into_owned();
    Err(anyhow!("invalid manifest:\n{path}"))
  }
}
