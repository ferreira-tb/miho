use crate::Release;
pub use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};

pub trait VersionExt {
  fn inc(&self, release: &Release) -> Version;
  fn inc_with_pre(&self, release: &Release, pre: Prerelease) -> Version;

  fn major(version: &Version) -> Version {
    Version {
      major: version.major + 1,
      minor: 0,
      patch: 0,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

  fn minor(version: &Version) -> Version {
    Version {
      major: version.major,
      minor: version.minor + 1,
      patch: 0,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

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
    match release {
      Release::Major => Self::major(self),
      Release::Minor => Self::minor(self),
      Release::Patch => Self::patch(self),
      Release::Literal(v) => v.clone(),
      _ => self.inc_with_pre(release, Prerelease::EMPTY),
    }
  }

  #[must_use]
  fn inc_with_pre(&self, release: &Release, pre: Prerelease) -> Version {
    let mut new_version = match release {
      Release::PreMajor => Self::major(self),
      Release::PreMinor => Self::minor(self),
      Release::PrePatch => Self::patch(self),
      Release::PreRelease => self.clone(),
      _ => self.inc(release),
    };

    new_version.pre = pre;

    new_version
  }
}
