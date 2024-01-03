use super::{manifest, Package};
use anyhow::{anyhow, Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use std::env;
use std::path::Path;

pub struct SearchBuilder {
  walker: WalkBuilder,
}

impl SearchBuilder {
  /// Create a new builder for a recursive directory search.
  pub fn new<P: AsRef<Path>>(path: P) -> Self {
    let mut walker = WalkBuilder::new(path);
    walker.add_ignore(".mihoignore");

    Self { walker }
  }

  /// Add a new search path.
  pub fn add<P: AsRef<Path>>(&mut self, path: P) -> &mut Self {
    self.walker.add(path);
    self
  }

  /// Search for packages recursively.
  ///
  /// This will respect `.gitignore` and `.mihoignore` files.
  pub fn search(self) -> Result<Vec<Package>> {
    let mut packages: Vec<Package> = vec![];
    let cwd = env::current_dir()?;
    let glob = self.build_globset()?;

    for result in self.walker.build() {
      let entry = result?;
      if self.is_match(&glob, &entry) {
        let path = cwd.join(entry.path().canonicalize()?);
        let path = path
          .to_str()
          .ok_or(anyhow!("invalid path:\n{}", path.display()))
          .with_context(|| "package search failed")?;

        let package = Package::new(path)?;
        packages.push(package);
      }
    }

    Ok(packages)
  }

  fn is_match(&self, glob: &GlobSet, entry: &DirEntry) -> bool {
    if !glob.is_match(entry.path()) {
      return false;
    }

    matches!(entry.file_type(), Some(t) if t.is_file())
  }

  fn build_globset(&self) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();

    builder.add(Glob::new(manifest::GLOB_CARGO_TOML)?);
    builder.add(Glob::new(manifest::GLOB_PACKAGE_JSON)?);
    builder.add(Glob::new(manifest::GLOB_TAURI_CONF_JSON)?);

    Ok(builder.build()?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_find_package() {
    let builder = SearchBuilder::new(".");
    let entries = builder.search().unwrap();
    let cwd = env::current_dir().unwrap();

    let toml = cwd.join("Cargo.toml").canonicalize().unwrap();
    let toml = toml.to_str().unwrap();

    if !entries.iter().any(|p| p.path.to_str().unwrap() == toml) {
      panic!("Cargo.toml not found");
    }
  }
}
