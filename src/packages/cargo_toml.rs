use super::{MihoPackage, Package, PackageType};
use crate::semver::{ReleaseType, Version};
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

impl MihoPackage for CargoToml {
  fn bump(package: &Package, release_type: &ReleaseType, pre_id: Option<&str>) -> Result<()> {
    let mut cargo_toml = CargoToml::read_as_value(&package.path)?;

    let new_version = CargoToml::new_version(package, release_type, pre_id)?;
    cargo_toml["version"] = Value::String(new_version.raw());

    let toml_string = toml::to_string_pretty(&cargo_toml)?;
    fs::write(&package.path, toml_string)?;

    Ok(())
  }

  fn to_package(path: &str) -> Result<Package> {
    let cargo_toml = CargoToml::read(path)?;

    let package = Package {
      name: cargo_toml.package.name,
      version: Version::new(&cargo_toml.package.version)?,
      package_type: PackageType::CargoToml,
      path: path.to_owned(),
    };

    Ok(package)
  }
}
