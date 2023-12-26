use super::{Package, PackageBuilder, PackageType};
use crate::semver::Version;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct PackageJson {
  pub name: String,
  pub version: String,
}

impl PackageBuilder for PackageJson {
  fn to_package(path: &str) -> Result<Package> {
    let json_string = fs::read_to_string(path)?;
    let package_json: PackageJson = serde_json::from_str(&json_string)?;

    let package = Package {
      name: package_json.name,
      version: Version::new(&package_json.version)?,
      package_type: PackageType::PackageJson,
      path: path.to_owned(),
    };

    Ok(package)
  }
}
