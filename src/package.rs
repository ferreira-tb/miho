mod agent;
pub mod builder;
pub mod dependency;
pub mod manifest;

pub use agent::Agent;
use manifest::ManifestType;
use semver::Version;
use std::fmt;
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

  #[must_use]
  pub fn agent(&self) -> Agent {
    self.manifest.agent()
  }

  /// Fetches metadata for all dependencies of the package.
  pub async fn dependency_tree(&self) -> crate::Result<dependency::Tree> {
    self.manifest.dependency_tree_builder().build().await
  }

  #[must_use]
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
