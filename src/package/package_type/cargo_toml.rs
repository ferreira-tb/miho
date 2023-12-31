use super::{PackageAction, PackageData};
use crate::package::Package;
use crate::semver::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::process::{Command, Stdio};
use toml::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoToml {
  pub package: CargoPackage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CargoPackage {
  pub name: String,
  pub version: String,
}

impl CargoToml {
  pub fn read(path: &str) -> Result<Self> {
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

impl PackageAction for CargoToml {
  fn bump(package: &Package) -> Result<()> {
    let mut cargo_toml = CargoToml::read_as_value(&package.path)?;

    let new_version = package.op.new_version.raw();
    cargo_toml["package"]["version"] = Value::String(new_version);

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

  fn data(path: &str) -> Result<PackageData> {
    let cargo_toml = CargoToml::read(path)?;
    let data = PackageData {
      name: cargo_toml.package.name,
      version: Version::new(&cargo_toml.package.version)?,
    };

    Ok(data)
  }
}
