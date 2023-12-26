use anyhow::{anyhow, Result};
use regex::Regex;

/// https://regex101.com/r/VX7uQk
pub const SEMVER_REGEX: &str =
  r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([a-zA-Z-]+)(?:\.(\d+)))?$";

#[derive(Debug)]
pub struct Version {
  pub major: usize,
  pub minor: usize,
  pub patch: usize,
  pub pre_id: Option<String>,
  pub pre_version: Option<usize>,
}

impl Version {
  pub fn new(raw: &str) -> Result<Self> {
    let raw = raw.trim().to_owned();
    if !is_valid(&raw) {
      return Err(anyhow!("Invalid semver: {}", raw));
    }

    let regex = Regex::new(SEMVER_REGEX).unwrap();
    let groups = regex.captures(&raw).unwrap();

    let major = groups.get(1).ok_or(anyhow!("Invalid major: {}", raw))?;
    let minor = groups.get(2).ok_or(anyhow!("Invalid minor: {}", raw))?;
    let patch = groups.get(3).ok_or(anyhow!("Invalid patch: {}", raw))?;

    let pre_id = match groups.get(4) {
      Some(id) => Some(id.as_str().to_owned()),
      None => None,
    };

    let pre_version = match groups.get(5) {
      Some(v) if pre_id.is_some() => Some(v.as_str().parse::<usize>()?),
      _ => None,
    };

    let version = Self {
      major: major.as_str().parse::<usize>()?,
      minor: minor.as_str().parse::<usize>()?,
      patch: patch.as_str().parse::<usize>()?,
      pre_id,
      pre_version,
    };

    Ok(version)
  }

  pub fn raw(&self) -> String {
    let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
    if let Some(id) = &self.pre_id {
      let pre = format!("-{}", id);
      version.push_str(pre.as_str());
    }

    if let Some(v) = &self.pre_version {
      let pre = format!(".{}", v);
      version.push_str(pre.as_str());
    }

    version
  }
}

pub fn is_valid(version: &str) -> bool {
  let regex = Regex::new(SEMVER_REGEX).unwrap();
  regex.is_match(version)
}

#[cfg(test)]
mod tests {
  use super::*;
  use anyhow::Result;

  #[test]
  fn should_be_valid() {
    let versions = ["1.0.0", "0.2.3-beta.1"];
    for version in versions {
      assert!(is_valid(version));
    }
  }

  #[test]
  fn should_be_invalid() {
    let versions = ["1", "3.5", "1.0.0-beta", "2.0.0-rc.1+build.123"];
    for version in versions {
      assert!(!is_valid(version));
    }
  }

  #[test]
  fn should_build_version_struct() -> Result<()> {
    let version = Version::new("6.2.3-beta.1")?;
    assert_eq!(6, version.major);
    assert_eq!(2, version.minor);
    assert_eq!(3, version.patch);
    assert_eq!("beta", version.pre_id.unwrap());
    assert_eq!(1, version.pre_version.unwrap());
    Ok(())
  }
}
