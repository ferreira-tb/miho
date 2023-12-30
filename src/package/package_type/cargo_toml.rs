use super::{PackageAction, PackageData};
use crate::package::Package;
use crate::semver::Version;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use toml::Value;

#[derive(Debug, Deserialize)]
pub struct CargoToml {
  pub package: CargoPackage,
}

#[derive(Debug, Deserialize)]
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
