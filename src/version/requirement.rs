use super::{Comparator, Version, VersionReq};

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
    self.comparators.iter().any(|c| c.matches(version))
  }
}
