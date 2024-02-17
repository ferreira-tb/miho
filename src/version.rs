mod comparator;
mod requirement;

use crate::release::Release;
pub use comparator::ComparatorExt;
pub use requirement::VersionReqExt;
pub use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};

pub trait VersionExt {
  fn as_comparator(&self, op: Op) -> Comparator;
  fn with_release(&self, release: &Release) -> Version;

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
  fn as_comparator(&self, op: Op) -> Comparator {
    Comparator::from_version(self, op)
  }

  #[must_use]
  fn with_release(&self, release: &Release) -> Version {
    macro_rules! build {
      ($build:expr, $version:expr) => {{
        let mut version = $version;
        version.build = $build.clone();
        version
      }};
    }

    macro_rules! pre {
      ($pre:expr, $build:expr, $version:expr) => {{
        let mut version = $version;
        version.pre = $pre.clone();
        version.build = $build.clone();
        version
      }};
    }

    match release {
      Release::Major(b) => build!(b, Version::major(self)),
      Release::Minor(b) => build!(b, Version::minor(self)),
      Release::Patch(b) => build!(b, Version::patch(self)),
      Release::PreMajor(p, b) => pre!(p, b, Version::major(self)),
      Release::PreMinor(p, b) => pre!(p, b, Version::minor(self)),
      Release::PrePatch(p, b) => pre!(p, b, Version::patch(self)),
      Release::PreRelease(p, b) => pre!(p, b, self.clone()),
      Release::Literal(v) => v.clone(),
    }
  }
}
