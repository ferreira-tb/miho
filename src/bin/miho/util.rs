use anyhow::Result;
use miho::package::builder::{Builder, Search};
use miho::package::Package;
use std::convert::AsRef;

pub fn search_packages<P: AsRef<str>>(path: &[P]) -> Result<Vec<Package>> {
  let mut paths: Vec<&str> = path.iter().map(AsRef::as_ref).collect();

  let last = paths.pop().unwrap_or(".");
  let mut search = Search::new(last);

  for path in paths {
    search.add(path);
  }

  let mut packages = search.execute()?;
  packages.sort_unstable();

  Ok(packages)
}
