use super::{Comparator, Op, Version};
use crate::release::Release;

pub trait ComparatorExt {
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
  fn with_release(&self, release: &Release) -> Comparator {
    let mut comparator = self.clone();

    match release {
      Release::Major(_) => {
        comparator.op = Op::Greater;
      }
      Release::Minor(_) => {
        comparator.op = Op::Caret;
      }
      Release::Patch(_) => {
        comparator.op = Op::Tilde;
      }
      _ => unreachable!(),
    }

    comparator
  }
}
