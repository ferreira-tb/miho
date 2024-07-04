use crate::package::dependency::{self, DependencyKind, DependencyTree};
use crate::package::manifest::{Handler, Manifest, ManifestBox};
use crate::package::{Agent, Package};
use crate::prelude::*;
use ahash::{HashMap, HashMapExt};
use taplo::formatter;
use toml::Value;

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

  const FILENAME: &'static str = "Cargo.toml";

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

    let contents = format(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn dependency_tree(&self) -> DependencyTree {
    let mut tree = DependencyTree::new(self.agent());

    macro_rules! add {
      ($dependencies:expr, $kind:ident) => {
        if let Some(deps) = $dependencies {
          let dependencies = parse_dependencies(deps);
          tree.add(&dependencies, DependencyKind::$kind);
        }
      };
    }

    add!(&self.dependencies, Normal);
    add!(&self.dev_dependencies, Development);
    add!(&self.build_dependencies, Build);

    tree
  }

  fn name(&self) -> &str {
    self.package.name.as_str()
  }

  fn update(&self, package: &Package, batch: &[dependency::Target]) -> Result<()> {
    let mut manifest = CargoToml::read_as_value(&package.path)?;

    for target in batch {
      let key = match target.dependency.kind {
        DependencyKind::Normal => "dependencies",
        DependencyKind::Development => "dev-dependencies",
        DependencyKind::Build => "build-dependencies",
        DependencyKind::Peer => continue,
      };

      let version = manifest
        .get_mut(key)
        .and_then(Value::as_table_mut)
        .and_then(|deps| deps.get_mut(&target.dependency.name));

      if let Some(value) = version {
        let mut comparator = target.comparator.to_string();
        if comparator.starts_with('^') {
          comparator.remove(0);
        }

        if value.is_str() {
          *value = Value::String(comparator);
        } else if value.is_table() {
          value["version"] = Value::String(comparator);
        }
      }
    }

    let contents = format(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn version(&self) -> Result<Version> {
    Version::parse(&self.package.version).map_err(Into::into)
  }
}

fn format(value: &Value) -> Result<String> {
  let contents = toml::to_string(value)?;
  let options = formatter::Options {
    column_width: 120,
    ..Default::default()
  };

  let contents = formatter::format(&contents, options);
  Ok(contents)
}

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

  if let Value::String(version) = value.get("version")? {
    if version == "*" {
      return None;
    }

    let path = value.get("path");
    let git = value.get("git");

    if path.is_none() && git.is_none() {
      return Some(version);
    }
  }

  None
}
