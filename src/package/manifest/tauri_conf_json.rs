use anyhow::{Result, anyhow};
use semver::Version;
use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::agent::Agent;
use crate::dependency;
use crate::package::Package;
use crate::package::manifest::{Handler, Manifest, ManifestBox};

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TauriConfJson {
  pub product_name: String,
  pub version: String,
}

impl Manifest for TauriConfJson {
  type Value = serde_json::Value;

  const FILENAME: &'static str = "tauri.conf.json";

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

  fn name(&self) -> &str {
    self.product_name.as_str()
  }

  fn update(&self, _: &Package, _: &[dependency::Target]) -> Result<()> {
    Err(anyhow!("{} does not support dependencies", Self::FILENAME))
  }

  fn version(&self) -> Result<Version> {
    Version::parse(&self.version).map_err(Into::into)
  }
}
