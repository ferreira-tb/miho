use super::Package;
use crate::semver::ReleaseType;
use anyhow::{anyhow, Context, Result};
use std::path::PathBuf;

pub struct PackageParser<'a> {
  entries: Vec<PathBuf>,
  release_type: Option<&'a ReleaseType>,
  pre_id: Option<&'a str>,
}

impl<'a> PackageParser<'a> {
  pub fn new(entries: Vec<PathBuf>) -> Self {
    Self {
      entries,
      release_type: None,
      pre_id: None,
    }
  }

  /// Define a release type for the packages.
  pub fn release(&mut self, release_type: &'a ReleaseType) -> &mut Self {
    self.release_type = Some(release_type);
    self
  }

  pub fn pre_id(&mut self, pre_id: &'a str) -> &mut Self {
    self.pre_id = Some(pre_id);
    self
  }

  pub fn parse(self) -> Result<Vec<Package>> {
    let release_type = match self.release_type {
      Some(rt) => rt,
      None => {
        return Err(anyhow!("Release type not specified"))
          .with_context(|| "Could not parse the packages")
      }
    };

    let mut packages: Vec<Package> = vec![];

    for entry in self.entries {
      let path = entry
        .to_str()
        .ok_or(anyhow!("Invalid package path"))
        .with_context(|| "Could not parse the packages")?;

      let path = path.to_owned();
      let pkg = Package::new(path, release_type, self.pre_id)?;
      packages.push(pkg);
    }

    Ok(packages)
  }
}
