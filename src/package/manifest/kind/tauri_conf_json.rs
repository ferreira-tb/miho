use crate::package::manifest::{Handler, Manifest, ManifestBox};
use crate::package::{Agent, Package};
use crate::version::Version;
use crate::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

const FILENAME_TAURI_CONF_JSON: &str = "tauri.conf.json";

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(super) struct TauriConfJson {
  pub product_name: String,
  pub version: String,
}

impl Manifest for TauriConfJson {
  type Value = serde_json::Value;

  fn read<P: AsRef<Path>>(path: P) -> Result<ManifestBox> {
    let contents = fs::read_to_string(path)?;
    let manifest: TauriConfJson = serde_json::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(path: P) -> Result<Self::Value> {
    let contents = fs::read_to_string(path)?;
    let manifest: Self::Value = serde_json::from_str(&contents)?;
    Ok(manifest)
  }
}

impl Handler for TauriConfJson {
  fn agent(&self) -> Agent {
    Agent::Tauri
  }

  fn bump(&self, package: &Package, version: Version) -> Result<()> {
    let mut manifest = TauriConfJson::read_as_value(&package.path)?;
    manifest["version"] = serde_json::Value::String(version.to_string());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn filename(&self) -> &str {
    FILENAME_TAURI_CONF_JSON
  }

  fn name(&self) -> &str {
    self.product_name.as_str()
  }

  fn update_dependencies(&self) -> Result<()> {
    Ok(())
  }

  fn version(&self) -> Result<Version> {
    let version = Version::parse(&self.version)?;
    Ok(version)
  }
}
