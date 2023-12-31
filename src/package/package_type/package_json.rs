use super::{PackageAction, PackageData};
use crate::package::Package;
use crate::semver::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
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

impl PackageAction for PackageJson {
  fn bump(package: &Package) -> Result<()> {
    let mut package_json = PackageJson::read_as_value(&package.path)?;

    let new_version = package.op.new_version.raw();
    package_json["version"] = Value::String(new_version);

    let json_string = serde_json::to_string_pretty(&package_json)?;
    fs::write(&package.path, json_string)?;

    Ok(())
  }

  fn data(path: &str) -> Result<PackageData> {
    let package_json = PackageJson::read(path)?;

    let data = PackageData {
      name: package_json.name,
      version: Version::new(&package_json.version)?,
    };

    Ok(data)
  }
}
