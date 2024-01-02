mod cargo_toml;
mod package_json;
mod tauri_conf_json;

use super::{transaction::Operation, Package};
use crate::semver::{ReleaseType, Version};
use anyhow::{anyhow, Result};
use cargo_toml::CargoToml;
use globset::Glob;
use package_json::PackageJson;
use tauri_conf_json::TauriConfJson;

pub(crate) const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
pub(crate) const GLOB_PACKAGE_JSON: &str = "**/package.json";
pub(crate) const GLOB_TAURI_CONF_JSON: &str = "**/tauri.conf.json";

#[derive(Copy, Clone, Debug)]
pub enum PackageType {
  CargoToml,
  PackageJson,
  TauriConfJson,
}

impl PackageType {
  pub fn bump(&self, package: &Package) -> Result<()> {
    match self {
      PackageType::CargoToml => CargoToml::bump(package),
      PackageType::PackageJson => PackageJson::bump(package),
      PackageType::TauriConfJson => TauriConfJson::bump(package),
    }
  }

  pub fn filename(&self) -> &str {
    match self {
      PackageType::CargoToml => "Cargo.toml",
      PackageType::PackageJson => "package.json",
      PackageType::TauriConfJson => "tauri.conf.json",
    }
  }

  pub fn glob(&self) -> &str {
    match self {
      PackageType::CargoToml => GLOB_CARGO_TOML,
      PackageType::PackageJson => GLOB_PACKAGE_JSON,
      PackageType::TauriConfJson => GLOB_TAURI_CONF_JSON,
    }
  }

  pub fn to_package(self, path: &str, rt: &ReleaseType, pre_id: Option<&str>) -> Result<Package> {
    let data = self.get_data(path)?;

    let operation = Operation {
      release_type: rt.clone(),
      pre_id: pre_id.map(|id| id.to_string()),
      new_version: data.version.inc(rt, pre_id)?,
    };

    let package = Package {
      name: data.name,
      version: data.version,
      package_type: self,
      path: path.to_string(),
      op: operation,
    };

    Ok(package)
  }

  fn get_data(&self, path: &str) -> Result<PackageData> {
    match self {
      PackageType::CargoToml => CargoToml::data(path),
      PackageType::PackageJson => PackageJson::data(path),
      PackageType::TauriConfJson => TauriConfJson::data(path),
    }
  }
}

pub(crate) fn parse_type<P: AsRef<str>>(path: P) -> Result<PackageType> {
  let path = path.as_ref();

  if is_of_type(path, PackageType::CargoToml)? {
    return Ok(PackageType::CargoToml);
  } else if is_of_type(path, PackageType::PackageJson)? {
    return Ok(PackageType::PackageJson);
  } else if is_of_type(path, PackageType::TauriConfJson)? {
    return Ok(PackageType::TauriConfJson);
  }

  Err(anyhow!("Could not parse package type for:\n{}", path))
}

pub(crate) fn is_of_type(path: &str, package_type: PackageType) -> Result<bool> {
  let glob = package_type.glob();
  Ok(Glob::new(glob)?.compile_matcher().is_match(path))
}

pub(crate) struct PackageData {
  pub name: String,
  pub version: Version,
}

pub(crate) trait PackageAction {
  fn bump(package: &Package) -> Result<()>;
  fn data(path: &str) -> Result<PackageData>;
}
