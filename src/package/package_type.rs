mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::PackageHandler;
use anyhow::{anyhow, Context, Result};
use cargo_toml::CargoToml;
use globset::Glob;
use package_json::PackageJson;
use std::path::Path;
use tauri_conf_json::TauriConfJson;

pub(super) const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
pub(super) const GLOB_PACKAGE_JSON: &str = "**/package.json";
pub(super) const GLOB_TAURI_CONF_JSON: &str = "**/tauri.conf.json";

#[derive(Copy, Clone, Debug)]
pub enum PackageType {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl PackageType {
  pub(super) fn read_source<P: AsRef<Path>>(&self, path: P) -> Result<Box<dyn PackageHandler>> {
    let path = path.as_ref();
    let source: Box<dyn PackageHandler> = match self {
      PackageType::CargoToml => Box::new(CargoToml::read(path)?),
      PackageType::PackageJson => Box::new(PackageJson::read(path)?),
      PackageType::TauriConfJson => Box::new(TauriConfJson::read(path)?),
    };

    Ok(source)
  }

  pub(super) fn glob(&self) -> &str {
    match self {
      PackageType::CargoToml => GLOB_CARGO_TOML,
      PackageType::PackageJson => GLOB_PACKAGE_JSON,
      PackageType::TauriConfJson => GLOB_TAURI_CONF_JSON,
    }
  }
}

impl TryFrom<&str> for PackageType {
  type Error = anyhow::Error;

  fn try_from(path: &str) -> Result<Self> {
    let variants = [
      PackageType::CargoToml,
      PackageType::PackageJson,
      PackageType::TauriConfJson,
    ];

    for variant in variants {
      let glob = variant.glob();
      let glob = Glob::new(glob)?.compile_matcher();
      if glob.is_match(path) {
        return Ok(variant);
      }
    }

    Err(anyhow!("invalid path:\n{}", path)).with_context(|| "could not parse package type")
  }
}
