use super::{Manifest, ManifestHandler};
use crate::package::Package;
use crate::versioning::semver::Version;
use anyhow::Result;
use miho_derive::{self, Manifest};
use serde::{Deserialize, Serialize};
use std::fs;
const FILENAME_CARGO_TOML: &str = "Cargo.toml";

#[derive(Manifest, Deserialize, Serialize)]
pub(super) struct CargoToml {
  pub package: CargoPackage,
}

#[derive(Deserialize, Serialize)]
pub(super) struct CargoPackage {
  pub name: String,
  pub version: String,
}

impl ManifestHandler for CargoToml {
  fn bump(&self, package: &Package, new_version: Version) -> Result<()> {
    let mut manifest = CargoToml::read_as_value(&package.manifest_path)?;
    manifest["package"]["version"] = toml::Value::String(new_version.raw());

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

  fn version(&self) -> Result<Version> {
    Version::new(&self.package.version)
  }
}
