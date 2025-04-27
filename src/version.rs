use anyhow::{Result, bail};
use semver::{BuildMetadata, Comparator, Op, Prerelease, Version, VersionReq};

use crate::release::Release;

pub trait VersionExt {
  fn as_comparator(&self, op: Op) -> Comparator;
  fn with_release(&self, release: &Release) -> Version;

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
  fn as_comparator(&self, op: Op) -> Comparator {
    Comparator::from_version(self, op)
  }

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

pub trait ComparatorExt {
  fn as_version(&self) -> Result<Version>;
  fn as_version_req(&self) -> VersionReq;
  fn normalize(&self, other: &mut Self);
  fn with_release(&self, release: &Release) -> Comparator;

  fn from_version(version: &Version, op: Op) -> Comparator {
    Comparator {
      op,
      major: version.major,
      minor: Some(version.minor),
      patch: Some(version.patch),
      pre: version.pre.clone(),
    }
  }
}

impl ComparatorExt for Comparator {
  fn as_version(&self) -> Result<Version> {
    let (Some(minor), Some(patch)) = (self.minor, self.patch) else {
      bail!("comparator {self} cannot be made into a version");
    };

    let mut version = Version::new(self.major, minor, patch);
    version.pre = self.pre.clone();

    Ok(version)
  }

  fn as_version_req(&self) -> VersionReq {
    VersionReq::from_comparator(self)
  }

  fn normalize(&self, other: &mut Self) {
    if self.minor.is_none() {
      other.minor = None;
    }

    if self.patch.is_none() {
      other.patch = None;
    }
  }

  fn with_release(&self, release: &Release) -> Comparator {
    let mut comparator = self.clone();
    match release {
      Release::Major(_) | Release::PreMajor(_, _) => {
        comparator.op = Op::GreaterEq;
      }
      Release::Minor(_) | Release::PreMinor(_, _) => {
        comparator.op = Op::Caret;
      }
      Release::Patch(_) | Release::PrePatch(_, _) => {
        comparator.op = Op::Tilde;
      }
      Release::Literal(_) | Release::PreRelease(_, _) => {
        comparator.op = Op::Exact;
      }
    }

    comparator
  }
}

pub trait VersionReqExt {
  fn from_comparator(comparator: &Comparator) -> VersionReq;
  fn matches_any(&self, version: &Version) -> bool;
}

impl VersionReqExt for VersionReq {
  fn from_comparator(comparator: &Comparator) -> VersionReq {
    let comparator = comparator.to_string();
    VersionReq::parse(&comparator).unwrap()
  }

  /// Evaluates if the version matches any of the comparators.
  fn matches_any(&self, version: &Version) -> bool {
    self
      .comparators
      .iter()
      .any(|c| c.matches(version))
  }
}
