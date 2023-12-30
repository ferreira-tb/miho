mod cargo_toml;
mod package_json;

use super::{transaction::Operation, Package};
use crate::semver::{ReleaseType, Version};
use anyhow::Result;
use cargo_toml::CargoToml;
use package_json::PackageJson;

pub const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
pub const GLOB_PACKAGE_JSON: &str = "**/package.json";

#[derive(Copy, Clone, Debug)]
pub enum PackageType {
  CargoToml,
  PackageJson,
}

impl PackageType {
  pub fn bump(&self, package: &Package) -> Result<()> {
    match self {
      PackageType::CargoToml => CargoToml::bump(package),
      PackageType::PackageJson => PackageJson::bump(package),
    }
  }

  pub fn filename(&self) -> &str {
    match self {
      PackageType::CargoToml => "Cargo.toml",
      PackageType::PackageJson => "package.json",
    }
  }

  pub fn glob(&self) -> &str {
    match self {
      PackageType::CargoToml => GLOB_CARGO_TOML,
      PackageType::PackageJson => GLOB_PACKAGE_JSON,
    }
  }

  pub fn to_package(self, path: &str, rt: &ReleaseType, pre_id: Option<&str>) -> Result<Package> {
    let data = self.get_data(path)?;

    let operation = Operation {
      release_type: *rt,
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
    }
  }
}

pub(crate) struct PackageData {
  pub name: String,
  pub version: Version,
}

pub(crate) trait PackageAction {
  fn bump(package: &Package) -> Result<()>;
  fn data(path: &str) -> Result<PackageData>;
}