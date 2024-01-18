use super::is_valid;
use super::release_type::ReleaseType;
use anyhow::{anyhow, bail, Result};
use regex::Regex;
use std::fmt;

/// It represents a version, based on the [SemVer](https://semver.org/) specification,
/// but stricter in the [accepted values](https://regex101.com/r/VX7uQk).
#[derive(Clone, Debug)]
pub struct Version {
  pub major: usize,
  pub minor: usize,
  pub patch: usize,
  pub pre_id: Option<String>,
  pub pre_version: Option<usize>,
}

impl Version {
  pub fn new<R: AsRef<str>>(raw: R) -> Result<Self> {
    let raw = raw.as_ref();
    let raw = raw.trim().to_owned();
    if !is_valid(&raw) {
      bail!("invalid semver: {}", raw);
    }

    let regex = Regex::new(super::SEMVER_REGEX).unwrap();
    let groups = regex.captures(&raw).unwrap();

    macro_rules! get {
      ($index:expr, $release_type:literal) => {
        groups
          .get($index)
          .ok_or_else(|| anyhow!("invalid {}: {}", $release_type, raw))
      };
    }

    let major = get!(1, "major")?;
    let minor = get!(2, "minor")?;
    let patch = get!(3, "patch")?;

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
        let pre_version = self
          .pre_version
          .ok_or_else(|| anyhow!("missing prerelease version"))?;

        version.pre_version = Some(pre_version + 1);
        version
      }
      ReleaseType::PreRelease => self.inc_pre(ReleaseType::Major, pre_id)?,
      ReleaseType::PreMajor => self.inc_pre(ReleaseType::Major, pre_id)?,
      ReleaseType::PreMinor => self.inc_pre(ReleaseType::Minor, pre_id)?,
      ReleaseType::PrePatch => self.inc_pre(ReleaseType::Patch, pre_id)?,
      ReleaseType::Literal(v) => Version::new(v)?,
    };

    Ok(version)
  }

  fn inc_pre(&self, release_type: ReleaseType, pre_id: Option<&str>) -> Result<Version> {
    if let Some(id) = pre_id {
      let mut version = self.inc(&release_type, None)?;
      version.pre_id = Some(id.to_string());
      version.pre_version = Some(1);
      Ok(version)
    } else {
      bail!("missing id for prerelease.")
    }
  }

  pub fn raw(&self) -> String {
    let mut version = format!("{}.{}.{}", self.major, self.minor, self.patch);
    if let Some(id) = &self.pre_id {
      let pre = format!("-{}", id);
      version.push_str(pre.as_str());

      if let Some(v) = &self.pre_version {
        let pre = format!(".{}", v);
        version.push_str(pre.as_str());
      }
    }

    version
  }
}

impl fmt::Display for Version {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.raw())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_create_version() {
    let version = Version::new("1.2.3").unwrap();

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.pre_id, None);
    assert_eq!(version.pre_version, None);
  }

  #[test]
  fn should_create_version_with_pre_id() {
    let version = Version::new("1.2.3-alpha.1").unwrap();

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.pre_id, Some("alpha".to_string()));
    assert_eq!(version.pre_version, Some(1));
  }

  #[test]
  fn should_increment_major() {
    let version = Version::new("1.2.3").unwrap();
    let version = version.inc(&ReleaseType::Major, None).unwrap();

    assert_eq!(version.major, 2);
    assert_eq!(version.minor, 0);
    assert_eq!(version.patch, 0);
    assert_eq!(version.pre_id, None);
    assert_eq!(version.pre_version, None);
  }

  #[test]
  fn should_increment_minor() {
    let version = Version::new("1.2.3").unwrap();
    let version = version.inc(&ReleaseType::Minor, None).unwrap();

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 3);
    assert_eq!(version.patch, 0);
    assert_eq!(version.pre_id, None);
    assert_eq!(version.pre_version, None);
  }

  #[test]
  fn should_increment_patch() {
    let version = Version::new("1.2.3").unwrap();
    let version = version.inc(&ReleaseType::Patch, None).unwrap();

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 4);
    assert_eq!(version.pre_id, None);
    assert_eq!(version.pre_version, None);
  }

  #[test]
  fn should_increment_prerelease() {
    let version = Version::new("1.2.3-alpha.1").unwrap();
    let version = version
      .inc(&ReleaseType::PreRelease, Some("alpha"))
      .unwrap();

    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.pre_id, Some("alpha".to_string()));
    assert_eq!(version.pre_version, Some(2));
  }
}
