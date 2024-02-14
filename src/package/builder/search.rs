use super::Builder;
use crate::error::Error;
use crate::package::manifest;
use crate::package::Package;
use crate::{bail, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use std::path::Path;

pub struct Search {
  walker: WalkBuilder,
}

impl Builder for Search {
  type Output = Result<Vec<Package>>;

  /// Searchs recursively for all packages in the given directory.
  ///
  /// This will respect `.gitignore` and `.mihoignore` files.
  fn execute(self) -> Self::Output {
    let mut packages = Vec::new();
    let glob = Self::build_globset();

    for result in self.walker.build() {
      let entry = match result {
        Ok(entry) => entry,
        Err(err) if err.is_io() => {
          let io_err = err.into_io_error().unwrap();
          bail!(Error::Io(io_err));
        }
        _ => continue,
      };

      if Self::is_match(&glob, &entry) {
        let Ok(path) = entry.path().canonicalize() else {
          bail!(Error::InvalidManifestPath {
            path: entry.path().to_string_lossy().into_owned(),
          });
        };

        if let Ok(package) = Package::new(path) {
          packages.push(package);
        }
      }
    }

    Ok(packages)
  }
}

impl Search {
  /// Creates a builder for a recursive directory search.
  #[must_use]
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    let mut walker = WalkBuilder::new(path);
    walker.add_ignore(".mihoignore");

    Self { walker }
  }

  /// Adds a file path to the underlying iterator.
  pub fn add<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
    self.walker.add(path);
    self
  }

  fn build_globset() -> GlobSet {
    let mut builder = GlobSetBuilder::new();

    macro_rules! add {
      ($kind:ident) => {
        let glob = manifest::Kind::$kind.glob();
        builder.add(Glob::new(glob).expect("hardcoded glob should always be valid"));
      };
    }

    add!(CargoToml);
    add!(PackageJson);
    add!(TauriConfJson);

    builder.build().unwrap()
  }

  fn is_match(glob: &GlobSet, entry: &DirEntry) -> bool {
    if !glob.is_match(entry.path()) {
      return false;
    }

    matches!(entry.file_type(), Some(t) if t.is_file())
  }
}
