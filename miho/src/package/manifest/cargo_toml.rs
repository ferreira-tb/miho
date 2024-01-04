use super::{Manifest, ManifestHandler};
use crate::package::Package;
use crate::versioning::semver::Version;
use anyhow::{anyhow, Result};
use miho_derive::{self, Manifest};
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Command, Stdio};

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
    let mut manifest = CargoToml::read_as_value(&package.path)?;
    manifest["package"]["version"] = toml::Value::String(new_version.raw());

    let contents = toml::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    // Ensures that `Cargo.lock` is updated immediately.
    let path = package
      .path
      .to_str()
      .ok_or(anyhow!("could not update Cargo.lock"))?;

    let manifest_path = format!("--manifest-path={}", path);
    Command::new("cargo")
      .args(["update", &manifest_path, &package.name])
      .stdout(Stdio::inherit())
      .stderr(Stdio::inherit())
      .output()?;

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
