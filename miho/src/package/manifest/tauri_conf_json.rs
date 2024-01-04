use super::{Manifest, ManifestHandler};
use crate::package::Package;
use crate::versioning::semver::Version;
use anyhow::Result;
use miho_derive::Manifest;
use serde::{Deserialize, Serialize};
use std::fs;

const FILENAME_TAURI_CONF_JSON: &str = "tauri.conf.json";

#[derive(Manifest, Deserialize, Serialize)]
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

impl ManifestHandler for TauriConfJson {
  fn bump(&self, package: &Package, new_version: Version) -> Result<()> {
    let mut manifest = TauriConfJson::read_as_value(&package.manifest_path)?;
    manifest["package"]["version"] = serde_json::Value::String(new_version.raw());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.manifest_path, contents)?;

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
