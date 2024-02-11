mod agent;
mod bump;
mod dependency;
mod manifest;
mod search;

pub use agent::Agent;
pub use bump::BumpBuilder;
pub use manifest::{ManifestHandler, ManifestType};
pub use search::SearchBuilder;
use semver::Version;
use std::fmt;
use std::path::{Path, PathBuf};

pub struct Package {
  pub name: String,
  pub version: Version,
  pub manifest_path: PathBuf,
  pub manifest: Box<dyn ManifestHandler>,
}

impl Package {
  /// Create a representation of the package from the manifest at `path`.
  pub fn new<P: AsRef<Path>>(manifest_path: P) -> crate::Result<Self> {
    let manifest_path = manifest_path.as_ref();
    let manifest_type = ManifestType::try_from(manifest_path)?;
    let manifest = manifest_type.read_source(manifest_path)?;

    let package = Self {
      name: manifest.name().to_owned(),
      version: manifest.version()?,
      manifest_path: manifest_path.to_path_buf(),
      manifest,
    };

    Ok(package)
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
