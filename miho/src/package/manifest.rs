mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::dependency::{self, Target};
use super::{Agent, Package};
use crate::prelude::*;
use cargo_toml::CargoToml;
use package_json::PackageJson;
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
  fn filename(&self) -> &str;
  fn name(&self) -> &str;
  fn update(&self, package: &Package, batch: &[Target]) -> Result<()>;
  fn version(&self) -> Result<Version>;

  fn dependency_tree(&self) -> dependency::Tree {
    dependency::Tree::new(self.agent())
  }
}

#[derive(Debug, EnumIter)]
pub enum Kind {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl Kind {
  const GLOB_CARGO_TOML: &'static str = "**/Cargo.toml";
  const GLOB_PACKAGE_JSON: &'static str = "**/package.json";
  const GLOB_TAURI_CONF_JSON: &'static str = "**/tauri.conf.json";

  pub(crate) fn read<P: AsRef<Path>>(&self, path: P) -> Result<ManifestBox> {
    match self {
      Kind::CargoToml => CargoToml::read(path),
      Kind::PackageJson => PackageJson::read(path),
      Kind::TauriConfJson => TauriConfJson::read(path),
    }
  }

  pub(crate) fn glob(&self) -> &str {
    match self {
      Kind::CargoToml => Self::GLOB_CARGO_TOML,
      Kind::PackageJson => Self::GLOB_PACKAGE_JSON,
      Kind::TauriConfJson => Self::GLOB_TAURI_CONF_JSON,
    }
  }
}

impl TryFrom<&Path> for Kind {
  type Error = anyhow::Error;

  fn try_from(path: &Path) -> Result<Self> {
    for kind in Kind::iter() {
      let glob = Glob::new(kind.glob())
        .expect("hardcoded glob should always be valid")
        .compile_matcher();

      if glob.is_match(path) {
        return Ok(kind);
      }
    }

    let path = path.to_string_lossy().into_owned();
    Err(anyhow!("invalid manifest:\n{path}"))
  }
}
