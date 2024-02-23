use crate::package::dependency;
use crate::package::manifest::{Handler, Manifest, ManifestBox};
use crate::package::{Agent, Package};
use crate::version::Version;
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TauriConfJson {
  pub package: TauriPackage,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TauriPackage {
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
    manifest["package"]["version"] = serde_json::Value::String(version.to_string());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn filename(&self) -> &str {
    Self::FILENAME
  }

  fn name(&self) -> &str {
    self.package.product_name.as_str()
  }

  fn update(&self, _: &Package, _: &[dependency::Target]) -> Result<()> {
    Err(anyhow!("{} does not support dependencies", Self::FILENAME))
  }

  fn version(&self) -> Result<Version> {
    Version::parse(&self.package.version).map_err(Into::into)
  }
}
