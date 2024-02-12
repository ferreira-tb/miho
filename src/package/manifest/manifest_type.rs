mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::{Manifest, ManifestHandler};
use cargo_toml::CargoToml;
use globset::Glob;
use package_json::PackageJson;
use std::path::Path;
use tauri_conf_json::TauriConfJson;

const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
const GLOB_PACKAGE_JSON: &str = "**/package.json";
const GLOB_TAURI_CONF_JSON: &str = "**/tauri.conf.json";

#[derive(Debug)]
pub enum ManifestType {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl ManifestType {
  pub(crate) fn read_source<P>(&self, path: P) -> crate::Result<Box<dyn ManifestHandler>>
  where
    P: AsRef<Path>,
  {
    match self {
      ManifestType::CargoToml => CargoToml::read(path),
      ManifestType::PackageJson => PackageJson::read(path),
      ManifestType::TauriConfJson => TauriConfJson::read(path),
    }
  }

  pub(crate) fn glob(&self) -> &str {
    match self {
      ManifestType::CargoToml => GLOB_CARGO_TOML,
      ManifestType::PackageJson => GLOB_PACKAGE_JSON,
      ManifestType::TauriConfJson => GLOB_TAURI_CONF_JSON,
    }
  }
}

impl TryFrom<&Path> for ManifestType {
  type Error = crate::error::Error;

  fn try_from(manifest_path: &Path) -> crate::Result<Self> {
    let variants = [
      ManifestType::CargoToml,
      ManifestType::PackageJson,
      ManifestType::TauriConfJson,
    ];

    for variant in variants {
      let glob = Glob::new(variant.glob())
        .expect("hardcoded glob should always be valid")
        .compile_matcher();

      if glob.is_match(manifest_path) {
        return Ok(variant);
      }
    }

    Err(Self::Error::InvalidManifestPath {
      path: manifest_path.to_string_lossy().into_owned(),
    })
  }
}
