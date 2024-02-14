use std::cmp::Ordering;
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
  Build,
  Development,
  Normal,
  Peer,
}

impl Kind {
  fn precedence(&self) -> u8 {
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
