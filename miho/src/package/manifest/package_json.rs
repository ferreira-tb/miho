use crate::package::dependency;
use crate::package::manifest::{Handler, Manifest, ManifestBox};
use crate::package::{Agent, Package};
use crate::prelude::*;
use serde_json::Value;
use std::fs;

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

  const FILENAME: &'static str = "package.json";

  fn read<P: AsRef<Path>>(path: P) -> Result<ManifestBox> {
    let contents = fs::read_to_string(path)?;
    let manifest: PackageJson = serde_json::from_str(&contents)?;
    Ok(Box::new(manifest))
  }

  fn read_as_value<P: AsRef<Path>>(path: P) -> Result<Self::Value> {
    let contents = fs::read_to_string(path)?;
    let manifest: Self::Value = serde_json::from_str(&contents)?;
    Ok(manifest)
  }
}

impl Handler for PackageJson {
  fn agent(&self) -> Agent {
    match &self.package_manager {
      Some(pm) if pm.starts_with("pnpm") => Agent::Pnpm,
      _ => Agent::Npm,
    }
  }

  fn bump(&self, package: &Package, version: Version) -> Result<()> {
    let mut manifest = PackageJson::read_as_value(&package.path)?;
    manifest["version"] = Value::String(version.to_string());

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn dependency_tree(&self) -> dependency::Tree {
    let mut tree = dependency::Tree::new(self.agent());

    macro_rules! add {
      ($deps:expr, $kind:ident) => {
        if let Some(deps) = $deps {
          tree.add(deps, dependency::Kind::$kind);
        }
      };
    }

    add!(&self.dependencies, Normal);
    add!(&self.dev_dependencies, Development);
    add!(&self.peer_dependencies, Peer);

    tree
  }

  fn filename(&self) -> &str {
    Self::FILENAME
  }

  fn name(&self) -> &str {
    self.name.as_str()
  }

  fn update(&self, package: &Package, batch: &[dependency::Target]) -> Result<()> {
    let mut manifest = PackageJson::read_as_value(&package.path)?;

    for target in batch {
      let key = match target.dependency.kind {
        dependency::Kind::Normal => "dependencies",
        dependency::Kind::Development => "devDependencies",
        dependency::Kind::Peer => "peerDependencies",
        dependency::Kind::Build => continue,
      };

      if let Some(deps) = manifest.get_mut(key).and_then(Value::as_object_mut) {
        let comparator = Value::String(target.comparator.to_string());
        deps.insert(target.dependency.name.clone(), comparator);
      }
    }

    let contents = serde_json::to_string_pretty(&manifest)?;
    fs::write(&package.path, contents)?;

    Ok(())
  }

  fn version(&self) -> Result<Version> {
    Version::parse(&self.version).map_err(Into::into)
  }
}
