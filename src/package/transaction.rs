use super::Package;
use crate::semver::{ReleaseType, Version};
use anyhow::Result;

pub struct Operation {
  pub release_type: ReleaseType,
  pub pre_id: Option<String>,
  pub new_version: Version,
}

pub struct Transaction {
  pub packages: Vec<Package>,
}

impl Transaction {
  pub fn new(packages: Vec<Package>) -> Self {
    Self { packages }
  }

  pub fn commit(&self) -> Result<()> {
    for package in &self.packages {
      package.package_type.bump(package)?;
    }

    Ok(())
  }
}
