mod agent;
mod bump;
mod dependency;
mod manifest;
mod search;

pub use agent::Agent;
pub use bump::BumpBuilder;
use dependency::DependencyTree;
pub use manifest::{ManifestHandler, ManifestType};
pub use search::SearchBuilder;
use semver::Version;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};

pub struct Package {
  pub name: String,
  pub version: Version,
  pub path: PathBuf,
  manifest: manifest::ManifestBox,
}

impl Package {
  /// Creates a representation of the package based on the manifest at `path`.
  pub fn new<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Self> {
    let path = manifest_path.as_ref();
    let manifest_type = ManifestType::try_from(path)?;
    let manifest = manifest_type.read_source(path)?;

    let package = Self {
      name: manifest.name().to_owned(),
      version: manifest.version()?,
      path: path.to_path_buf(),
      manifest,
    };

    Ok(package)
  }

  pub fn agent(&self) -> Agent {
    self.manifest.agent()
  }

  /// Fetches metadata for all dependencies of the package.
  ///
  /// This is potentially a long-running operation.
  pub async fn dependency_tree(&self) -> crate::Result<DependencyTree> {
    self.manifest.dependency_tree_builder().build().await
  }

  pub fn filename(&self) -> &str {
    self.manifest.filename()
  }
}

impl fmt::Display for Package {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let filename = self.filename();
    write!(f, "{} ({})", self.name, filename.to_lowercase())
  }
}

impl PartialEq for Package {
  fn eq(&self, other: &Self) -> bool {
    self.path == other.path
  }
}

impl Eq for Package {}

impl Hash for Package {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.path.hash(state);
  }
}
