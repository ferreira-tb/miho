use crate::package::{Package, PackageHandler};
use crate::versioning::semver::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::{Command, Stdio};
use toml::Value;

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

impl CargoToml {
  pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
    let toml_string = fs::read_to_string(path)?;
    let cargo_toml: CargoToml = toml::from_str(&toml_string)?;
    Ok(cargo_toml)
  }

  pub fn read_as_value(path: &str) -> Result<Value> {
    let toml_string = fs::read_to_string(path)?;
    let cargo_toml: Value = toml::from_str(&toml_string)?;
    Ok(cargo_toml)
  }
}

impl PackageHandler for CargoToml {
  fn bump(&self, package: &Package, new_version: Version) -> Result<()> {
    let mut cargo_toml = CargoToml::read_as_value(&package.path)?;
    cargo_toml["package"]["version"] = Value::String(new_version.raw());

    let toml_string = toml::to_string_pretty(&cargo_toml)?;
    fs::write(&package.path, toml_string)?;

    // Ensures that `Cargo.lock` is updated immediately.
    let manifest_path = format!("--manifest-path={}", package.path);
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
