use crate::package::dependency;
use crate::package::manifest::{Handler, Manifest, ManifestBox};
use crate::package::{Agent, Package};
use crate::version::Version;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::Value;

const FILENAME_CARGO_TOML: &str = "Cargo.toml";

#[derive(Deserialize)]
pub(super) struct CargoToml {
  pub package: CargoPackage,
  pub dependencies: Option<HashMap<String, Value>>,

  #[serde(rename(deserialize = "dev-dependencies"))]
  pub dev_dependencies: Option<HashMap<String, Value>>,

  #[serde(rename(deserialize = "build-dependencies"))]
  pub build_dependencies: Option<HashMap<String, Value>>,
}

#[derive(Deserialize)]
pub(super) struct CargoPackage {
  pub name: String,
  pub version: String,
}

impl Manifest for CargoToml {
  type Value = toml::Value;

  fn read<P: AsRef<Path>>(path: P) -> Result<ManifestBox> {
    let contents = fs::read_to_string(path)?;
    let manifest: CargoToml = toml::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(path: P) -> Result<Self::Value> {
    let contents = fs::read_to_string(path)?;
    let manifest: Self::Value = toml::from_str(&contents)?;
    Ok(manifest)
  }
}

impl Handler for CargoToml {
  fn agent(&self) -> Agent {
    Agent::Cargo
  }

  fn bump(&self, package: &Package, version: Version) -> Result<()> {
    let mut manifest = CargoToml::read_as_value(&package.path)?;
    manifest["package"]["version"] = Value::String(version.to_string());

    let contents = toml::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn dependency_tree(&self) -> dependency::Tree {
    let mut tree = dependency::Tree::new(self.agent());

    macro_rules! add {
      ($dependencies:expr, $kind:ident) => {
        if let Some(deps) = $dependencies {
          let dependencies = parse_dependencies(deps);
          tree.add(&dependencies, dependency::Kind::$kind);
        }
      };
    }

    add!(&self.dependencies, Normal);
    add!(&self.dev_dependencies, Development);
    add!(&self.build_dependencies, Build);

    tree
  }

  fn filename(&self) -> &str {
    FILENAME_CARGO_TOML
  }

  fn name(&self) -> &str {
    self.package.name.as_str()
  }

  fn update(&self, package: &Package, batch: Vec<dependency::Update>) -> Result<()> {
    let mut manifest = CargoToml::read_as_value(&package.path)?;

    for update in batch {
      let key = match update.dependency.kind {
        dependency::Kind::Normal => "dependencies",
        dependency::Kind::Development => "dev-dependencies",
        dependency::Kind::Build => "build-dependencies",
        dependency::Kind::Peer => continue,
      };

      let version = manifest
        .get_mut(key)
        .and_then(Value::as_table_mut)
        .and_then(|deps| deps.get_mut(&update.dependency.name));

      if let Some(value) = version {
        let mut target = update.target.to_string();
        if target.starts_with('^') {
          target.remove(0);
        }
        
        if value.is_str() {
          *value = Value::String(target);
        } else if value.is_table() {
          value["version"] = Value::String(target);
        }
      }
    }

    let contents = toml::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn version(&self) -> Result<Version> {
    Version::parse(&self.package.version).map_err(Into::into)
  }
}

// Could we refactor this so less cloning is needed?
fn parse_dependencies(deps: &HashMap<String, Value>) -> HashMap<String, String> {
  let mut dependencies = HashMap::with_capacity(deps.len());
  for (name, version) in deps {
    if let Some(version) = parse_version(version) {
      dependencies.insert(name.clone(), version.clone());
    }
  }

  dependencies
}

fn parse_version(value: &Value) -> Option<&String> {
  if let Value::String(version) = value {
    return Some(version);
  }

  if let Value::String(version) = &value["version"] {
    return Some(version);
  }

  None
}
