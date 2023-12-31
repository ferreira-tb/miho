use super::{PackageAction, PackageData};
use crate::package::Package;
use crate::semver::Version;
use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct TauriConfJson {
  pub package: TauriPackage,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
pub struct TauriPackage {
  pub product_name: String,
  pub version: String,
}

impl TauriConfJson {
  pub fn read(path: &str) -> Result<Self> {
    let json_string = fs::read_to_string(path)?;
    let package_json: TauriConfJson = serde_json::from_str(&json_string)?;
    Ok(package_json)
  }

  pub fn read_as_value(path: &str) -> Result<Value> {
    let json_string = fs::read_to_string(path)?;
    let package_json: Value = serde_json::from_str(&json_string)?;
    Ok(package_json)
  }
}

impl PackageAction for TauriConfJson {
  fn bump(package: &Package) -> Result<()> {
    let mut tauri_conf = TauriConfJson::read_as_value(&package.path)?;

    let new_version = package.op.new_version.raw();
    tauri_conf["package"]["version"] = Value::String(new_version);

    let json_string = serde_json::to_string_pretty(&tauri_conf)?;
    fs::write(&package.path, json_string)?;

    Ok(())
  }

  fn data(path: &str) -> Result<PackageData> {
    let tauri_conf = TauriConfJson::read(path)?;

    let data = PackageData {
      name: tauri_conf.package.product_name,
      version: Version::new(&tauri_conf.package.version)?,
    };

    Ok(data)
  }
}
