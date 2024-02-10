use super::{Manifest, ManifestHandler};
use crate::error::Result;
use crate::package::Package;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const FILENAME_PACKAGE_JSON: &str = "package.json";

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub(super) struct PackageJson {
  pub name: String,
  pub version: String,
  pub package_manager: Option<String>,

  pub scripts: Option<HashMap<String, String>>,

  pub dependencies: Option<HashMap<String, String>>,
  pub dev_dependencies: Option<HashMap<String, String>>,
  pub peer_dependencies: Option<HashMap<String, String>>,
}

impl Manifest for PackageJson {
  type Value = serde_json::Value;

  fn read<P: AsRef<Path>>(manifest_path: P) -> Result<Box<dyn ManifestHandler>> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: PackageJson = serde_json::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(manifest_path: P) -> Result<Self::Value> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: Self::Value = serde_json::from_str(&contents)?;
    Ok(manifest)
  }
}

impl ManifestHandler for PackageJson {
  fn bump(&self, package: &Package, version: Version) -> Result<()> {
    let mut manifest = PackageJson::read_as_value(&package.manifest_path)?;
    manifest["version"] = serde_json::Value::String(version.to_string());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.manifest_path, contents)?;

    Ok(())
  }

  fn filename(&self) -> &str {
    FILENAME_PACKAGE_JSON
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn update(&self) -> Result<()> {
    Ok(())
  }

  fn version(&self) -> Result<Version> {
    let version = Version::parse(&self.version)?;
    Ok(version)
  }
}
