use super::{Manifest, ManifestHandler};
use crate::package::Package;
use anyhow::Result;
use miho_derive::Manifest;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;

const FILENAME_TAURI_CONF_JSON: &str = "tauri.conf.json";

#[derive(Debug, Manifest, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub(super) struct TauriConfJson {
  pub product_name: String,
  pub version: String,
}

impl ManifestHandler for TauriConfJson {
  fn bump(&self, package: &Package, version: Version) -> Result<()> {
    let mut manifest = TauriConfJson::read_as_value(&package.manifest_path)?;
    manifest["version"] = serde_json::Value::String(version.to_string());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.manifest_path, contents)?;

    Ok(())
  }

  fn filename(&self) -> &str {
    FILENAME_TAURI_CONF_JSON
  }

  fn name(&self) -> &str {
    self.product_name.as_str()
  }

  fn version(&self) -> Result<Version> {
    let version = Version::parse(&self.version)?;
    Ok(version)
  }
}
