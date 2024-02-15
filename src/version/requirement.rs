use super::{Comparator, VersionReq};

pub trait VersionReqExt {
  fn from_comparator(comparator: &Comparator) -> VersionReq;
}

impl VersionReqExt for VersionReq {
  fn from_comparator(comparator: &Comparator) -> VersionReq {
    let comparator = comparator.to_string();
    VersionReq::parse(&comparator).unwrap()
  }
}
