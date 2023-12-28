use anyhow::{anyhow, Result};
use regex::Regex;

/// <https://regex101.com/r/VX7uQk>
pub const SEMVER_REGEX: &str =
  r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([a-zA-Z-]+)(?:\.(\d+)))?$";

pub enum ReleaseType {
  Major,
  Minor,
  Patch,
  PreMajor,
  PreMinor,
  PrePatch,
  PreRelease,
}

#[derive(Clone, Debug)]
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
    
    let pre_id = groups.get(4).map(|id| id.as_str().to_owned());

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

  /// Increment the version by a release type.
  pub fn inc(&self, release_type: &ReleaseType, pre_id: Option<&str>) -> Result<Version> {
    let version = match release_type {
      ReleaseType::Major => Version {
        major: self.major + 1,
        minor: 0,
        patch: 0,
        pre_id: None,
        pre_version: None,
      },
      ReleaseType::Minor => Version {
        major: self.major,
        minor: self.minor + 1,
        patch: 0,
        pre_id: None,
        pre_version: None,
      },
      ReleaseType::Patch => Version {
        major: self.major,
        minor: self.minor,
        patch: self.patch + 1,
        pre_id: None,
        pre_version: None,
      },
      ReleaseType::PreRelease if self.pre_id.is_none() => {
        self.inc_pre(ReleaseType::Patch, pre_id)?
      }
      ReleaseType::PreRelease if self.pre_version.is_some() => {
        let mut version = self.clone();
        let pre_version = self.pre_version.unwrap();
        version.pre_version = Some(pre_version + 1);
        version
      }
      ReleaseType::PreRelease => self.inc_pre(ReleaseType::Major, pre_id)?,
      ReleaseType::PreMajor => self.inc_pre(ReleaseType::Major, pre_id)?,
      ReleaseType::PreMinor => self.inc_pre(ReleaseType::Minor, pre_id)?,
      ReleaseType::PrePatch => self.inc_pre(ReleaseType::Patch, pre_id)?,
    };

    Ok(version)
  }

  fn inc_pre(&self, release_type: ReleaseType, pre_id: Option<&str>) -> Result<Version> {
    match pre_id {
      Some(id) => {
        let mut version = self.inc(&release_type, None)?;
        version.pre_id = Some(id.to_string());
        version.pre_version = Some(1);
        Ok(version)
      }
      None => Err(anyhow!("Missing id for prerelease.")),
    }
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

pub fn to_release_type(raw: &str) -> Result<ReleaseType> {
  let release_type = raw.to_lowercase();
  let release_type = match release_type.trim() {
    "major" => ReleaseType::Major,
    "minor" => ReleaseType::Minor,
    "patch" => ReleaseType::Patch,
    "premajor" => ReleaseType::PreMajor,
    "preminor" => ReleaseType::PreMinor,
    "prepatch" => ReleaseType::PrePatch,
    "prerelease" => ReleaseType::PreRelease,
    _ => return Err(anyhow!("Cannot convert {} into a release type.", raw)),
  };

  Ok(release_type)
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
