use crate::package::manifest::{Manifest, ManifestHandler};
use crate::package::Package;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const FILENAME_CARGO_TOML: &str = "Cargo.toml";

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CargoToml {
  pub package: CargoPackage,
}

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CargoPackage {
  pub name: String,
  pub version: String,
}

impl Manifest for CargoToml {
  type Value = toml::Value;

  fn read<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Box<dyn ManifestHandler>> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: CargoToml = toml::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Self::Value> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: Self::Value = toml::from_str(&contents)?;
    Ok(manifest)
  }
}

impl ManifestHandler for CargoToml {
  fn bump(&self, package: &Package, version: Version) -> crate::Result<()> {
    let mut manifest = CargoToml::read_as_value(&package.manifest_path)?;
    manifest["package"]["version"] = toml::Value::String(version.to_string());

    let contents = toml::to_string_pretty(&manifest)?;
    fs::write(&package.manifest_path, contents)?;

    Ok(())
  }

  fn filename(&self) -> &str {
    FILENAME_CARGO_TOML
  }

  fn name(&self) -> &str {
    self.package.name.as_str()
  }

  fn update(&self) -> crate::Result<()> {
    Ok(())
  }

  fn version(&self) -> crate::Result<Version> {
    let version = Version::parse(&self.package.version)?;
    Ok(version)
  }
}
