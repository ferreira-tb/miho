pub mod bump;
pub mod git;
pub mod packages;
pub mod semver;
pub mod stdio;

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use std::env;
use std::path::PathBuf;

pub fn search() -> Result<Vec<PathBuf>> {
  let mut walker = WalkBuilder::new(".");
  walker.add_ignore(".mihoignore");

  let mut entries: Vec<PathBuf> = vec![];
  let glob = build_globset()?;
  let cwd = env::current_dir()?;

  for result in walker.build() {
    let entry = result?;
    if is_match(&glob, &entry) {
      let path = cwd.join(entry.path().canonicalize()?);
      entries.push(path);
    }
  }

  Ok(entries)
}

fn build_globset() -> Result<GlobSet> {
  let mut builder = GlobSetBuilder::new();

  builder.add(Glob::new(packages::GLOB_PACKAGE_JSON)?);
  builder.add(Glob::new(packages::GLOB_CARGO_TOML)?);

  Ok(builder.build()?)
}

fn is_match(glob: &GlobSet, entry: &DirEntry) -> bool {
  if !glob.is_match(entry.path()) {
    return false;
  }
  
  matches!(entry.file_type(), Some(t) if t.is_file())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_find_package() -> Result<()> {
    let entries = search()?;
    let cwd = env::current_dir()?;
    let toml = cwd.join("Cargo.toml").canonicalize()?;

    if !entries.iter().any(|p| p.to_str() == toml.to_str()) {
      panic!("Cargo.toml not found");
    }

    Ok(())
  }
}
