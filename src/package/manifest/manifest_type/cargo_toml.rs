use crate::package::dependency::{self, TreeBuilder};
use crate::package::manifest::{Handler, Manifest, ManifestBox};
use crate::package::{Agent, Package};
use semver::Version;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const FILENAME_CARGO_TOML: &str = "Cargo.toml";

#[derive(Deserialize)]
pub(super) struct CargoToml {
  pub package: CargoPackage,
  pub dependencies: Option<HashMap<String, toml::Value>>,

  #[serde(rename(deserialize = "dev-dependencies"))]
  pub dev_dependencies: Option<HashMap<String, toml::Value>>,

  #[serde(rename(deserialize = "build-dependencies"))]
  pub build_dependencies: Option<HashMap<String, toml::Value>>,
}

#[derive(Deserialize)]
pub(super) struct CargoPackage {
  pub name: String,
  pub version: String,
}

impl Manifest for CargoToml {
  type Value = toml::Value;

  fn read<P: AsRef<Path>>(path: P) -> crate::Result<ManifestBox> {
    let contents = fs::read_to_string(path)?;
    let manifest: CargoToml = toml::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(path: P) -> crate::Result<Self::Value> {
    let contents = fs::read_to_string(path)?;
    let manifest: Self::Value = toml::from_str(&contents)?;
    Ok(manifest)
  }
}

impl Handler for CargoToml {
  fn agent(&self) -> Agent {
    Agent::Cargo
  }

  fn bump(&self, package: &Package, version: Version) -> crate::Result<()> {
    let mut manifest = CargoToml::read_as_value(&package.path)?;
    manifest["package"]["version"] = toml::Value::String(version.to_string());

    let contents = toml::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn dependency_tree_builder(&self) -> TreeBuilder {
    let mut builder = TreeBuilder::new(self.agent());

    macro_rules! add {
      ($dependencies:expr, $kind:ident) => {
        if let Some(deps) = $dependencies {
          let dependencies = parse_dependencies(deps);
          builder.add(&dependencies, dependency::Kind::$kind);
        }
      };
    }

    add!(&self.dependencies, Normal);
    add!(&self.dev_dependencies, Development);
    add!(&self.build_dependencies, Build);

    builder
  }

  fn filename(&self) -> &str {
    FILENAME_CARGO_TOML
  }

  fn name(&self) -> &str {
    self.package.name.as_str()
  }

  fn update_dependencies(&self) -> crate::Result<()> {
    Ok(())
  }

  fn version(&self) -> crate::Result<Version> {
    let version = Version::parse(&self.package.version)?;
    Ok(version)
  }
}

fn parse_dependencies(deps: &HashMap<String, toml::Value>) -> HashMap<String, String> {
  let mut dependencies = HashMap::with_capacity(deps.len());
  for (name, version) in deps {
    if let Some(version) = parse_version(version) {
      dependencies.insert(name.clone(), version.clone());
    }
  }

  dependencies
}

// Could we refactor this so less cloning is needed?
fn parse_version(value: &toml::Value) -> Option<&String> {
  if let toml::Value::String(version) = value {
    return Some(version);
  }

  if let toml::Value::String(version) = &value["version"] {
    return Some(version);
  }

  None
}
