mod kind;
mod tree;

use crate::return_if_ne;
use crate::version::{Comparator, Version};
pub use kind::Kind;
use std::cmp::Ordering;
pub use tree::Tree;

#[derive(Debug)]
pub struct Dependency {
  pub name: String,
  pub version: Comparator,
  pub kind: Kind,
  versions: Vec<Version>,
}

impl Dependency {
  /// Returns the maximum version that satisfies the version constraint.
  #[must_use]
  pub fn max(&self) -> Option<&Version> {
    self.max_with_comparator(&self.version)
  }

  /// Returns the maximum version that satisfies a given version constraint.
  #[must_use]
  pub fn max_with_comparator(&self, comparator: &Comparator) -> Option<&Version> {
    self
      .versions
      .iter()
      .filter(|v| comparator.matches(v))
      .max_by(|a, b| Version::cmp_precedence(a, b))
  }
}

impl PartialEq for Dependency {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.version == other.version && self.kind == other.kind
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
