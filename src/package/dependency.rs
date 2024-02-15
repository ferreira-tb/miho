mod kind;
mod tree;

use crate::return_if_ne;
use crate::version::{Comparator, Version, VersionReq};
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
      .filter(|v| requirement.matches(v))
      .max_by(|a, b| Version::cmp_precedence(a, b))
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
    return_if_ne!(self.name.cmp(&other.name));

    self.kind.cmp(&other.kind)
  }
}
