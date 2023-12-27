use crate::packages::cargo_toml::CargoToml;
use crate::packages::package_json::PackageJson;
use crate::packages::{MihoPackage, Package, PackageType};
use crate::semver::ReleaseType;
use anyhow::Result;

pub fn bump(packages: Vec<Package>, release_type: ReleaseType, pre_id: Option<&str>) -> Result<()> {
  for package in packages {
    match package.package_type {
      PackageType::CargoToml => CargoToml::bump(&package, &release_type, pre_id)?,
      PackageType::PackageJson => PackageJson::bump(&package, &release_type, pre_id)?,
    }
  }

  Ok(())
}
