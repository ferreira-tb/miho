use super::{Comparator, Op, Version};

pub trait ComparatorExt {
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

impl ComparatorExt for Comparator {}
