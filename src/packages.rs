pub mod cargo_toml;
pub mod package_json;

use self::cargo_toml::CargoToml;
use self::package_json::PackageJson;
use crate::semver::{ReleaseType, Version};
use anyhow::{anyhow, Result};
use globset::Glob;
use std::fmt;
use std::path::PathBuf;

pub const GLOB_CARGO_TOML: &str = "**/Cargo.toml";
pub const GLOB_PACKAGE_JSON: &str = "**/package.json";

#[derive(Debug)]
pub enum PackageType {
  CargoToml,
  PackageJson,
}

pub struct Package {
  pub package_type: PackageType,
  pub name: String,
  pub version: Version,
  pub path: String,
}

impl Package {
  pub fn new(path: &str) -> Result<Self> {
    let package_type = parse_package_type(path)?;
    let package = match package_type {
      PackageType::CargoToml => CargoToml::to_package(path)?,
      PackageType::PackageJson => PackageJson::to_package(path)?,
    };

    Ok(package)
  }

  pub fn filename(&self) -> String {
    let string = match &self.package_type {
      PackageType::CargoToml => "Cargo.toml",
      PackageType::PackageJson => "package.json",
    };

    String::from(string)
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} ({})", self.name, self.filename().to_lowercase())
  }
}

pub trait MihoPackage {
  fn bump(package: &Package, release_type: &ReleaseType, pre_id: Option<&str>) -> Result<()>;
  fn to_package(path: &str) -> Result<Package>;

  fn new_version(
    package: &Package,
    release_type: &ReleaseType,
    pre_id: Option<&str>,
  ) -> Result<Version> {
    let version = package.version.inc(release_type, pre_id)?;
    Ok(version)
  }
}

pub fn create_packages(entries: Vec<PathBuf>) -> Result<Vec<Package>> {
  let mut packages: Vec<Package> = vec![];

  for entry in entries {
    let path = entry.to_str().ok_or(anyhow!("Invalid package path"))?;
    let pkg = Package::new(path)?;
    packages.push(pkg);
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
