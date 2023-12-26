use super::{Package, PackageBuilder, PackageType};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub(crate) struct CargoToml {
  pub package: CargoPackage,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct CargoPackage {
  pub name: String,
  pub version: String,
}

impl PackageBuilder for CargoToml {
  fn to_package(path: &str) -> Result<Package> {
    let toml_string = fs::read_to_string(path)?;
    let cargo_toml: CargoToml = toml::from_str(&toml_string)?;

    let package = Package {
      name: cargo_toml.package.name,
      version: cargo_toml.package.version,
      package_type: PackageType::CargoToml,
      path: path.to_owned(),
    };

    Ok(package)
  }
}
