use std::fmt;

pub enum Choice {
  All,
  Some,
  None,
}

impl fmt::Display for Choice {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::All => write!(f, "All"),
      Self::Some => write!(f, "Some"),
      Self::None => write!(f, "None"),
    }
  }
}
