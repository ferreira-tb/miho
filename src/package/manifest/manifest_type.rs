mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::{Manifest, ManifestHandler};
use cargo_toml::CargoToml;
use globset::Glob;
use package_json::PackageJson;
use std::path::Path;
use tauri_conf_json::TauriConfJson;

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
      ManifestType::CargoToml => "**/Cargo.toml",
      ManifestType::PackageJson => "**/package.json",
      ManifestType::TauriConfJson => "**/tauri.conf.json",
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
