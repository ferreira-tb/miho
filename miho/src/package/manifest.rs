mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::dependency;
use super::{Agent, Package};
use crate::version::Version;
use anyhow::{anyhow, Result};
use cargo_toml::CargoToml;
use globset::Glob;
use package_json::PackageJson;
use std::path::Path;
use tauri_conf_json::TauriConfJson;

pub(super) type ManifestBox = Box<dyn Handler + Send + Sync>;

const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
const GLOB_PACKAGE_JSON: &str = "**/package.json";
const GLOB_TAURI_CONF_JSON: &str = "**/tauri.conf.json";

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
  fn update(&self, package: &Package, batch: &[dependency::Target]) -> Result<()>;
  fn version(&self) -> Result<Version>;

  fn dependency_tree(&self) -> dependency::Tree {
    dependency::Tree::new(self.agent())
  }
}

#[derive(Debug)]
pub enum Kind {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl Kind {
  pub(crate) fn read<P>(&self, path: P) -> Result<ManifestBox>
  where
    P: AsRef<Path>,
  {
    match self {
      Kind::CargoToml => CargoToml::read(path),
      Kind::PackageJson => PackageJson::read(path),
      Kind::TauriConfJson => TauriConfJson::read(path),
    }
  }

  pub(crate) fn glob(&self) -> &str {
    match self {
      Kind::CargoToml => GLOB_CARGO_TOML,
      Kind::PackageJson => GLOB_PACKAGE_JSON,
      Kind::TauriConfJson => GLOB_TAURI_CONF_JSON,
    }
  }
}

impl TryFrom<&Path> for Kind {
  type Error = anyhow::Error;

  fn try_from(path: &Path) -> Result<Self> {
    let variants = [Kind::CargoToml, Kind::PackageJson, Kind::TauriConfJson];

    for variant in variants {
      let glob = Glob::new(variant.glob())
        .expect("hardcoded glob should always be valid")
        .compile_matcher();

      if glob.is_match(path) {
        return Ok(variant);
      }
    }

    let path = path.to_string_lossy().into_owned();
    Err(anyhow!("invalid manifest:\n{path}"))
  }
}
