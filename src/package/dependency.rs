mod tree;

use crate::release::Release;
use crate::return_if_ne;
use crate::version::{Comparator, ComparatorExt, Version, VersionExt, VersionReq, VersionReqExt};
use std::cmp::Ordering;
use std::fmt;
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

  #[must_use]
  pub fn into_update(self, release: &Option<Release>) -> Option<Update> {
    let target = self.target_cmp(release);

    if matches!(target, Some(ref t) if *t != self.comparator) {
      let update = Update {
        dependency: self,
        target: target.unwrap(),
      };

      Some(update)
    } else {
      None
    }
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
  Build,
  Development,
  Normal,
  Peer,
}

impl Kind {
  fn precedence(self) -> u8 {
    match self {
      Self::Normal => 0,
      Self::Development => 1,
      Self::Build => 2,
      Self::Peer => 3,
    }
  }
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Build => write!(f, "build"),
      Self::Development => write!(f, "dev"),
      Self::Normal => write!(f, ""),
      Self::Peer => write!(f, "peer"),
    }
  }
}

impl PartialOrd for Kind {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Kind {
  fn cmp(&self, other: &Self) -> Ordering {
    self.precedence().cmp(&other.precedence())
  }
}

#[derive(Debug)]
pub struct Update {
  pub dependency: Dependency,
  pub target: Comparator,
}
