use anyhow::Result;
use semver::{BuildMetadata, Prerelease, Version};

#[derive(Clone, Debug)]
pub enum Release {
  Major,
  Minor,
  Patch,
  PreMajor,
  PreMinor,
  PrePatch,
  PreRelease,
  Literal(Version),
}

impl Release {
  pub fn increment(&self, version: &Version) -> Version {
    match self {
      Release::Major => self.major(version),
      Release::Minor => self.minor(version),
      Release::Patch => self.patch(version),
      Release::Literal(v) => v.clone(),
      _ => self.increment_pre(version, Prerelease::EMPTY),
    }
  }

  pub fn increment_pre(&self, version: &Version, pre: Prerelease) -> Version {
    let mut new_version = match self {
      Release::PreMajor => self.major(version),
      Release::PreMinor => self.minor(version),
      Release::PrePatch => self.patch(version),
      Release::PreRelease => version.clone(),
      _ => self.increment(version),
    };

    new_version.pre = pre;

    new_version
  }

  fn major(&self, version: &Version) -> Version {
    Version {
      major: version.major + 1,
      minor: 0,
      patch: 0,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

  fn minor(&self, version: &Version) -> Version {
    Version {
      major: version.major,
      minor: version.minor + 1,
      patch: 0,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

  fn patch(&self, version: &Version) -> Version {
    Version {
      major: version.major,
      minor: version.minor,
      patch: version.patch + 1,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }
}

impl TryFrom<&str> for Release {
  type Error = anyhow::Error;

  fn try_from(val: &str) -> Result<Self> {
    let release = val.to_lowercase();
    let release = match release.trim() {
      "major" => Release::Major,
      "minor" => Release::Minor,
      "patch" => Release::Patch,
      "premajor" => Release::PreMajor,
      "preminor" => Release::PreMinor,
      "prepatch" => Release::PrePatch,
      "prerelease" => Release::PreRelease,
      rt => {
        let version = Version::parse(rt)?;
        Release::Literal(version)
      }
    };

    Ok(release)
  }
}