mod kind;
mod tree;

use crate::release::Release;
use crate::return_if_ne;
use crate::version::{Comparator, ComparatorExt, Version, VersionExt, VersionReq, VersionReqExt};
pub use kind::Kind;
use std::cmp::Ordering;
pub use tree::Tree;

#[derive(Debug)]
pub struct Dependency {
  pub name: String,
  pub comparator: Comparator,
  pub kind: Kind,
  versions: Vec<Version>,
}

impl Dependency {
  #[must_use]
  pub fn latest(&self) -> Option<&Version> {
    self
      .versions
      .iter()
      .max_by(|a, b| Version::cmp_precedence(a, b))
  }

  #[must_use]
  pub fn latest_with_req(&self, requirement: &VersionReq) -> Option<&Version> {
    self
      .versions
      .iter()
      .filter(|v| requirement.matches_any(v))
      .max_by(|a, b| Version::cmp_precedence(a, b))
  }

  #[must_use]
  pub fn target_cmp(&self, release: &Option<Release>) -> Option<Comparator> {
    let comparator = &self.comparator;
    let requirement = if let Some(r) = release {
      comparator.with_release(r).as_version_req()
    } else {
      comparator.as_version_req()
    };

    self.latest_with_req(&requirement).and_then(|target| {
      let target_cmp = target.as_comparator(comparator.op);
      if target_cmp == *comparator {
        None
      } else {
        Some(target_cmp)
      }
    })
  }
}

impl PartialEq for Dependency {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.comparator == other.comparator && self.kind == other.kind
  }
}

impl Eq for Dependency {}

impl PartialOrd for Dependency {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Dependency {
  fn cmp(&self, other: &Self) -> Ordering {
    return_if_ne!(self.kind.cmp(&other.kind));
    self.name.cmp(&other.name)
  }
}
