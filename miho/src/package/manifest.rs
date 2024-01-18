mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::Package;
use crate::versioning::semver::Version;
use anyhow::{bail, Result};
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

  fn read<P: AsRef<Path>>(manifest_path: P) -> Result<Box<dyn ManifestHandler>>;
  fn read_as_value<P: AsRef<Path>>(manifest_path: P) -> Result<Self::Value>;
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
  pub(super) fn read_source<P: AsRef<Path>>(
    &self,
    manifest_path: P,
  ) -> Result<Box<dyn ManifestHandler>> {
    let manifest_path = manifest_path.as_ref();
    match self {
      ManifestType::CargoToml => CargoToml::read(manifest_path),
      ManifestType::PackageJson => PackageJson::read(manifest_path),
      ManifestType::TauriConfJson => TauriConfJson::read(manifest_path),
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

  fn try_from(manifest_path: &Path) -> Result<Self> {
    let variants = [
      ManifestType::CargoToml,
      ManifestType::PackageJson,
      ManifestType::TauriConfJson,
    ];

    for variant in variants {
      let glob = variant.glob();
      let glob = Glob::new(glob)?.compile_matcher();
      if glob.is_match(manifest_path) {
        return Ok(variant);
      }
    }

    bail!("could not parse manifest type")
  }
}
