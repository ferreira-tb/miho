mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::Package;
use crate::versioning::semver::Version;
use anyhow::{anyhow, Result};
use cargo_toml::CargoToml;
use globset::Glob;
use package_json::PackageJson;
use std::path::Path;
use tauri_conf_json::TauriConfJson;

pub(super) const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
pub(super) const GLOB_PACKAGE_JSON: &str = "**/package.json";
pub(super) const GLOB_TAURI_CONF_JSON: &str = "**/tauri.conf.json";

trait Manifest {
  type Value;

  fn read<P: AsRef<Path>>(path: P) -> Result<Box<dyn ManifestHandler>>;
  fn read_as_value<P: AsRef<Path>>(path: P) -> Result<Self::Value>;
}

pub(super) trait ManifestHandler {
  fn bump(&self, package: &Package, new_version: Version) -> Result<()>;
  fn filename(&self) -> &str;
  fn name(&self) -> &str;
  fn version(&self) -> Result<Version>;
}

#[derive(Copy, Clone, Debug)]
pub enum ManifestType {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl ManifestType {
  pub(super) fn read_source<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn ManifestHandler>> {
    let path = path.as_ref();
    match self {
      ManifestType::CargoToml => CargoToml::read(path),
      ManifestType::PackageJson => PackageJson::read(path),
      ManifestType::TauriConfJson => TauriConfJson::read(path),
    }
  }

  pub(super) fn glob(&self) -> &str {
    match self {
      ManifestType::CargoToml => GLOB_CARGO_TOML,
      ManifestType::PackageJson => GLOB_PACKAGE_JSON,
      ManifestType::TauriConfJson => GLOB_TAURI_CONF_JSON,
    }
  }
}

impl TryFrom<&Path> for ManifestType {
  type Error = anyhow::Error;

  fn try_from(path: &Path) -> Result<Self> {
    let variants = [
      ManifestType::CargoToml,
      ManifestType::PackageJson,
      ManifestType::TauriConfJson,
    ];

    for variant in variants {
      let glob = variant.glob();
      let glob = Glob::new(glob)?.compile_matcher();
      if glob.is_match(path) {
        return Ok(variant);
      }
    }

    Err(anyhow!("could not parse manifest type"))
  }
}
