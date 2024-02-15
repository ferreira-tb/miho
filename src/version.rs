mod comparator;
mod requirement;

use crate::release::Release;
pub use comparator::ComparatorExt;
pub use requirement::VersionReqExt;
pub use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};

pub trait VersionExt {
  fn inc(&self, release: &Release) -> Version;
  fn inc_with_pre(&self, release: &Release, pre: Prerelease) -> Version;

  #[must_use]
  fn major(version: &Version) -> Version {
    Version {
      major: version.major + 1,
      minor: 0,
      patch: 0,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

  #[must_use]
  fn minor(version: &Version) -> Version {
    Version {
      major: version.major,
      minor: version.minor + 1,
      patch: 0,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

  #[must_use]
  fn patch(version: &Version) -> Version {
    Version {
      major: version.major,
      minor: version.minor,
      patch: version.patch + 1,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }
}

impl VersionExt for Version {
  #[must_use]
  fn inc(&self, release: &Release) -> Version {
    increment(self, release, None)
  }

  #[must_use]
  fn inc_with_pre(&self, release: &Release, pre: Prerelease) -> Version {
    increment(self, release, Some(pre))
  }
}

#[must_use]
fn increment(version: &Version, release: &Release, pre: Option<Prerelease>) -> Version {
  let mut new_version = match release {
    Release::Major | Release::PreMajor => Version::major(version),
    Release::Minor | Release::PreMinor => Version::minor(version),
    Release::Patch | Release::PrePatch => Version::patch(version),
    Release::PreRelease => version.clone(),
    Release::Literal(v) => v.clone(),
  };

  if let Some(pre) = pre {
    new_version.pre = pre;
  }

  new_version
}
