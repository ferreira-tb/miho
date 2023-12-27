use super::{MihoPackage, Package, PackageType};
use crate::semver::{ReleaseType, Version};
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct PackageJson {
  pub name: String,
  pub version: String,
}

impl PackageJson {
  pub fn read(path: &str) -> Result<Self> {
    let json_string = fs::read_to_string(path)?;
    let package_json: PackageJson = serde_json::from_str(&json_string)?;
    Ok(package_json)
  }

  pub fn read_as_value(path: &str) -> Result<Value> {
    let json_string = fs::read_to_string(path)?;
    let package_json: Value = serde_json::from_str(&json_string)?;
    Ok(package_json)
  }
}

impl MihoPackage for PackageJson {
  fn bump(package: &Package, release_type: &ReleaseType, pre_id: Option<&str>) -> Result<()> {
    let mut package_json = PackageJson::read_as_value(&package.path)?;

    let new_version = PackageJson::new_version(package, release_type, pre_id)?;
    package_json["version"] = Value::String(new_version.raw());

    let json_string = serde_json::to_string_pretty(&package_json)?;
    fs::write(&package.path, json_string)?;

    Ok(())
  }

  fn to_package(path: &str) -> Result<Package> {
    let package_json = PackageJson::read(path)?;

    let package = Package {
      name: package_json.name,
      version: Version::new(&package_json.version)?,
      package_type: PackageType::PackageJson,
      path: path.to_owned(),
    };

    Ok(package)
  }
}
