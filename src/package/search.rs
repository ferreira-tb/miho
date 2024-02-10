use super::{manifest, Package};
use crate::{bail, Error, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use std::path::Path;

pub struct SearchBuilder {
  walker: WalkBuilder,
}

impl SearchBuilder {
  /// Creates a builder for a recursive directory search.
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    let mut walker = WalkBuilder::new(path);
    walker.add_ignore(".mihoignore");

    Self { walker }
  }

  /// Adds a new path to the search.
  pub fn add<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
    self.walker.add(path);
    self
  }

  /// Searchs recursively for all packages in the given directory.
  ///
  /// This will respect `.gitignore` and `.mihoignore` files.
  pub fn search(self) -> Result<Vec<Package>> {
    let mut packages: Vec<Package> = vec![];
    let glob = build_globset();

    for result in self.walker.build() {
      let entry = match result {
        Ok(entry) => entry,
        Err(err) if err.is_io() => {
          if let Some(io_err) = err.into_io_error() {
            bail!(Error::Io(io_err));
          } else {
            continue;
          }
        }
        _ => continue,
      };

      if is_match(&glob, &entry) {
        let Ok(manifest_path) = entry.path().canonicalize() else {
          bail!(Error::InvalidManifestPath {
            path: entry.path().to_string_lossy().into_owned(),
          });
        };

        if let Ok(package) = Package::new(manifest_path) {
          packages.push(package);
        }
      }
    }

    Ok(packages)
  }
}

fn build_globset() -> GlobSet {
  let mut builder = GlobSetBuilder::new();

  macro_rules! add {
    ($glob:expr) => {
      builder.add(Glob::new($glob).expect("hardcoded glob should always be valid"));
    };
  }

  add!(manifest::GLOB_CARGO_TOML);
  add!(manifest::GLOB_PACKAGE_JSON);
  add!(manifest::GLOB_TAURI_CONF_JSON);

  builder.build().unwrap()
}

fn is_match(glob: &GlobSet, entry: &DirEntry) -> bool {
  if !glob.is_match(entry.path()) {
    return false;
  }

  matches!(entry.file_type(), Some(t) if t.is_file())
}
