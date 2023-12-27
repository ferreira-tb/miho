mod cargo_toml;
mod package_json;

use self::cargo_toml::CargoToml;
use self::package_json::PackageJson;
use crate::semver::Version;
use anyhow::{anyhow, Result};
use globset::Glob;
use std::path::PathBuf;

pub const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
pub const GLOB_PACKAGE_JSON: &str = "**/package.json";

#[derive(Debug)]
pub enum PackageType {
  CargoToml,
  PackageJson,
}

#[derive(Debug)]
pub struct Package {
  pub id: u32,
  pub package_type: PackageType,
  pub name: String,
  pub version: Version,
  pub path: String,
}

impl Package {
  pub fn new(id: u32, path: &str) -> Result<Self> {
    let package_type = parse_package_type(path)?;
    let package = match package_type {
      PackageType::CargoToml => CargoToml::to_package(id, path)?,
      PackageType::PackageJson => PackageJson::to_package(id, path)?,
    };

    Ok(package)
  }
}

pub trait PackageBuilder {
  fn to_package(id: u32, path: &str) -> Result<Package>;
}

pub fn create_packages(entries: Vec<PathBuf>) -> Result<Vec<Package>> {
  let mut id: u32 = 0;
  let mut packages: Vec<Package> = vec![];

  for entry in entries {
    let path = entry.to_str().ok_or(anyhow!("Invalid package path"))?;
    let pkg = Package::new(id, path)?;
    packages.push(pkg);
    id = id + 1;
  }

  Ok(packages)
}

pub fn parse_package_type(path: &str) -> Result<PackageType> {
  if is_package(PackageType::CargoToml, path)? {
    return Ok(PackageType::CargoToml);
  } else if is_package(PackageType::PackageJson, path)? {
    return Ok(PackageType::PackageJson);
  }

  Err(anyhow!("Could not parse package type for:\n{}", path))
}

pub fn is_package(package_type: PackageType, path: &str) -> Result<bool> {
  let glob = match package_type {
    PackageType::CargoToml => GLOB_CARGO_TOML,
    PackageType::PackageJson => GLOB_PACKAGE_JSON,
  };

  Ok(Glob::new(glob)?.compile_matcher().is_match(path))
}
