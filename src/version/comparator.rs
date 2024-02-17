use super::{Comparator, Op, Version, VersionReq, VersionReqExt};
use crate::release::Release;

pub trait ComparatorExt {
  fn as_version_req(&self) -> VersionReq;
  fn with_release(&self, release: &Release) -> Comparator;

  #[must_use]
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
  #[must_use]
  fn as_version_req(&self) -> VersionReq {
    VersionReq::from_comparator(self)
  }

  #[must_use]
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
