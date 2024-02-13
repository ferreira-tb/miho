use crate::package::dependency::{self, TreeBuilder};
use crate::package::manifest::{Manifest, ManifestBox, ManifestHandler};
use crate::package::{Agent, Package};
use semver::Version;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const FILENAME_PACKAGE_JSON: &str = "package.json";

#[derive(Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub(super) struct PackageJson {
  pub name: String,
  pub version: String,
  pub package_manager: Option<String>,

  pub dependencies: Option<HashMap<String, String>>,
  pub dev_dependencies: Option<HashMap<String, String>>,
  pub peer_dependencies: Option<HashMap<String, String>>,
}

impl Manifest for PackageJson {
  type Value = serde_json::Value;

  fn read<P: AsRef<Path>>(path: P) -> crate::Result<ManifestBox> {
    let contents = fs::read_to_string(path)?;
    let manifest: PackageJson = serde_json::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(path: P) -> crate::Result<Self::Value> {
    let contents = fs::read_to_string(path)?;
    let manifest: Self::Value = serde_json::from_str(&contents)?;
    Ok(manifest)
  }
}

impl ManifestHandler for PackageJson {
  fn agent(&self) -> Agent {
    match &self.package_manager {
      Some(pm) if pm.starts_with("pnpm") => Agent::Pnpm,
      Some(pm) if pm.starts_with("yarn") => Agent::Yarn,
      _ => Agent::Npm,
    }
  }

  fn bump(&self, package: &Package, version: Version) -> crate::Result<()> {
    let mut manifest = PackageJson::read_as_value(&package.path)?;
    manifest["version"] = serde_json::Value::String(version.to_string());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn dependency_tree_builder(&self) -> TreeBuilder {
    let mut builder = TreeBuilder::new(self.agent());

    macro_rules! add {
      ($deps:expr, $kind:ident) => {
        if let Some(deps) = $deps {
          builder.add(deps, dependency::Kind::$kind);
        }
      };
    }

    add!(&self.dependencies, Normal);
    add!(&self.dev_dependencies, Development);
    add!(&self.peer_dependencies, Peer);

    builder
  }

  fn filename(&self) -> &str {
    FILENAME_PACKAGE_JSON
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn update_dependencies(&self) -> crate::Result<()> {
    Ok(())
  }

  fn version(&self) -> crate::Result<Version> {
    let version = Version::parse(&self.version)?;
    Ok(version)
  }
}
