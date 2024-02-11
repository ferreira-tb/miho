use crate::package::dependency::DependencyTree;
use crate::package::manifest::{Manifest, ManifestHandler};
use crate::package::{Agent, Package};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(serialize = "snake_case", deserialize = "camelCase"))]
pub(super) struct PackageJson {
  pub name: String,
  pub version: String,
  pub package_manager: Option<String>,

  pub scripts: Option<HashMap<String, String>>,

  pub dependencies: Option<HashMap<String, String>>,
  pub dev_dependencies: Option<HashMap<String, String>>,
  pub peer_dependencies: Option<HashMap<String, String>>,
}

impl Manifest for PackageJson {
  type Value = serde_json::Value;

  fn read<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Box<dyn ManifestHandler>> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: PackageJson = serde_json::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Self::Value> {
    let contents = fs::read_to_string(manifest_path)?;
    let manifest: Self::Value = serde_json::from_str(&contents)?;
    Ok(manifest)
  }
}

impl ManifestHandler for PackageJson {
  fn agent(&self) -> Agent {
    if let Some(package_manager) = &self.package_manager {
      if package_manager.starts_with("pnpm") {
        return Agent::Pnpm;
      } else if package_manager.starts_with("yarn") {
        return Agent::Yarn;
      }
    }

    Agent::Npm
  }

  fn bump(&self, package: &Package, version: Version) -> crate::Result<()> {
    let mut manifest = PackageJson::read_as_value(&package.manifest_path)?;
    manifest["version"] = serde_json::Value::String(version.to_string());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.manifest_path, contents)?;

    Ok(())
  }

  fn dependencies(&self) -> DependencyTree {
    let mut builder = DependencyTree::builder();

    if let Some(deps) = &self.dependencies {
      builder.normal(deps);
    }

    if let Some(deps) = &self.dev_dependencies {
      builder.dev(deps);
    }

    if let Some(deps) = &self.peer_dependencies {
      builder.peer(deps);
    }

    builder.build()
  }

  fn filename(&self) -> &str {
    "package.json"
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn update(&self) -> crate::Result<()> {
    Ok(())
  }

  fn version(&self) -> crate::Result<Version> {
    let version = Version::parse(&self.version)?;
    Ok(version)
  }
}
