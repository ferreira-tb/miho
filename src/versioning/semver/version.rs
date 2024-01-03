use super::is_valid;
use super::release_type::ReleaseType;
use anyhow::{anyhow, bail, Result};
use regex::Regex;

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

    let major = groups.get(1).ok_or(anyhow!("invalid major: {}", raw))?;
    let minor = groups.get(2).ok_or(anyhow!("invalid minor: {}", raw))?;
    let patch = groups.get(3).ok_or(anyhow!("invalid patch: {}", raw))?;

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
      ReleaseType::Literal(v) => Version::new(v)?,
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
      None => Err(anyhow!("missing id for prerelease.")),
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
