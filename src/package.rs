mod package_type;
pub mod transaction;

use self::transaction::Operation;
use crate::semver::{ReleaseType, Version};
use anyhow::{anyhow, Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
pub use package_type::PackageType;
use std::path::PathBuf;
use std::{env, fmt};

/// Represents a package of a given type.
pub struct Package {
  pub package_type: PackageType,
  pub name: String,
  pub version: Version,
  pub path: String,
  pub op: Operation,
}

impl Package {
  /// Creates a representation of the package at `path`.
  ///
  /// # Errors
  /// This will return an error if the file in the path provided is not a valid package.
  pub fn new<T: AsRef<str>>(
    path: T,
    release_type: &ReleaseType,
    pre_id: Option<&str>,
  ) -> Result<Self> {
    let path = path.as_ref();
    let package_type = parse_package_type(path)?;
    let package = package_type.to_package(path, release_type, pre_id)?;
    Ok(package)
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let filename = self.package_type.filename();
    write!(f, "{} ({})", self.name, filename.to_lowercase())
  }
}

/// Searches recursively for packages in the current directory.
///
/// This will respect `.gitignore` and `.mihoignore` files.
pub fn search() -> Result<Vec<PathBuf>> {
  let mut walker = WalkBuilder::new(".");
  walker.add_ignore(".mihoignore");

  let mut entries: Vec<PathBuf> = vec![];
  let glob = build_globset()?;
  let cwd = env::current_dir()?;

  for result in walker.build() {
    let entry = result?;
    if is_match(&glob, &entry) {
      let path = cwd.join(entry.path().canonicalize()?);
      entries.push(path);
    }
  }

  Ok(entries)
}

fn is_match(glob: &GlobSet, entry: &DirEntry) -> bool {
  if !glob.is_match(entry.path()) {
    return false;
  }

  matches!(entry.file_type(), Some(t) if t.is_file())
}

fn build_globset() -> Result<GlobSet> {
  let mut builder = GlobSetBuilder::new();

  builder.add(Glob::new(package_type::GLOB_PACKAGE_JSON)?);
  builder.add(Glob::new(package_type::GLOB_CARGO_TOML)?);
  builder.add(Glob::new(package_type::GLOB_TAURI_CONF_JSON)?);

  Ok(builder.build()?)
}

pub fn parse_packages(
  entries: Vec<PathBuf>,
  release_type: &ReleaseType,
  pre_id: Option<&str>,
) -> Result<Vec<Package>> {
  let mut packages: Vec<Package> = vec![];

  for entry in entries {
    let path = entry
      .to_str()
      .ok_or(anyhow!("Invalid package path"))
      .with_context(|| "Could not parse the packages")?;

    let path = path.to_owned();
    let pkg = Package::new(path, release_type, pre_id)?;
    packages.push(pkg);
  }

  Ok(packages)
}

pub fn parse_package_type(path: &str) -> Result<PackageType> {
  if is_package_of_type(path, PackageType::CargoToml)? {
    return Ok(PackageType::CargoToml);
  } else if is_package_of_type(path, PackageType::PackageJson)? {
    return Ok(PackageType::PackageJson);
  }

  Err(anyhow!("Could not parse package type for:\n{}", path))
}

pub fn is_package_of_type(path: &str, package_type: PackageType) -> Result<bool> {
  let glob = package_type.glob();
  Ok(Glob::new(glob)?.compile_matcher().is_match(path))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_find_package() {
    let entries = search().unwrap();
    let cwd = env::current_dir().unwrap();
    let toml = cwd.join("Cargo.toml").canonicalize().unwrap();

    if !entries.iter().any(|p| p.to_str() == toml.to_str()) {
      panic!("Cargo.toml not found");
    }
  }
}
