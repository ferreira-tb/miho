use crate::package::manifest::{Manifest, ManifestHandler};
use crate::package::Package;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILENAME_TAURI_CONF_JSON: &str = "tauri.conf.json";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub(super) struct TauriConfJson {
  pub product_name: String,
  pub version: String,
}

impl Manifest for TauriConfJson {
  type Value = serde_json::Value;

  fn read<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Box<dyn ManifestHandler>> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: TauriConfJson = serde_json::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Self::Value> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: Self::Value = serde_json::from_str(&contents)?;
    Ok(manifest)
  }
}

impl ManifestHandler for TauriConfJson {
  fn bump(&self, package: &Package, version: Version) -> crate::Result<()> {
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

  fn update(&self) -> crate::Result<()> {
    Ok(())
  }

  fn version(&self) -> crate::Result<Version> {
    let version = Version::parse(&self.version)?;
    Ok(version)
  }
}
