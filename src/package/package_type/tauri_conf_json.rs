use crate::package::{Package, PackageHandler};
use crate::versioning::semver::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

const FILENAME_TAURI_CONF_JSON: &str = "tauri.conf.json";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub(super) struct TauriConfJson {
  pub package: TauriPackage,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub(super) struct TauriPackage {
  pub product_name: String,
  pub version: String,
}

impl TauriConfJson {
  pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
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

impl PackageHandler for TauriConfJson {
  fn bump(&self, package: &Package, new_version: Version) -> Result<()> {
    let mut tauri_conf = TauriConfJson::read_as_value(&package.path)?;
    tauri_conf["package"]["version"] = Value::String(new_version.raw());

    let json_string = serde_json::to_string_pretty(&tauri_conf)?;
    fs::write(&package.path, json_string)?;

    Ok(())
  }

  fn filename(&self) -> &str {
    FILENAME_TAURI_CONF_JSON
  }

  fn name(&self) -> &str {
    self.package.product_name.as_str()
  }

  fn version(&self) -> Result<Version> {
    Version::new(&self.package.version)
  }
}
