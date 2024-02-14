use anyhow::Result;
use miho::package::builder::{self, Builder};
use miho::package::Package;
use std::convert::AsRef;

pub fn search_packages<P: AsRef<str>>(path: &[P]) -> Result<Vec<Package>> {
  let mut paths: Vec<&str> = path.iter().map(AsRef::as_ref).collect();

  let last = paths.pop().unwrap_or(".");
  let mut search = builder::Search::new(last);

  for path in paths {
    search.add(path);
  }

  let mut packages = search.execute()?;
  packages.sort_unstable_by(|a, b| a.cmp(&b));

  Ok(packages)
}
