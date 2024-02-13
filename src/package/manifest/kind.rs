mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::{Manifest, ManifestBox};
use cargo_toml::CargoToml;
use globset::Glob;
use package_json::PackageJson;
use std::path::Path;
use tauri_conf_json::TauriConfJson;

const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
const GLOB_PACKAGE_JSON: &str = "**/package.json";
const GLOB_TAURI_CONF_JSON: &str = "**/tauri.conf.json";

#[derive(Debug)]
pub enum Kind {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl Kind {
  pub(crate) fn read_source<P>(&self, path: P) -> crate::Result<ManifestBox>
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
  type Error = crate::error::Error;

  fn try_from(path: &Path) -> crate::Result<Self> {
    let variants = [Kind::CargoToml, Kind::PackageJson, Kind::TauriConfJson];

    for variant in variants {
      let glob = Glob::new(variant.glob())
        .expect("hardcoded glob should always be valid")
        .compile_matcher();

      if glob.is_match(path) {
        return Ok(variant);
      }
    }

    Err(Self::Error::InvalidManifestPath {
      path: path.to_string_lossy().into_owned(),
    })
  }
}
