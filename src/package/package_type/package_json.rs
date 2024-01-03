use crate::package::{Package, PackageHandler};
use crate::versioning::semver::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::Path;

const FILENAME_PACKAGE_JSON: &str = "package.json";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub(super) struct PackageJson {
  pub name: String,
  pub version: String,
}

impl PackageJson {
  pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
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

impl PackageHandler for PackageJson {
  fn bump(&self, package: &Package, new_version: Version) -> Result<()> {
    let mut package_json = PackageJson::read_as_value(&package.path)?;
    package_json["version"] = Value::String(new_version.raw());

    let json_string = serde_json::to_string_pretty(&package_json)?;
    fs::write(&package.path, json_string)?;

    Ok(())
  }

  fn filename(&self) -> &str {
    FILENAME_PACKAGE_JSON
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn version(&self) -> Result<Version> {
    Version::new(&self.version)
  }
}
