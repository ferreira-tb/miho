use super::Builder;
use crate::Result;

pub struct Update;

impl Builder for Update {
  type Output = ();

  fn execute(self) -> Result<Self::Output> {
    unimplemented!()
  }
}

impl Update {
  #[must_use]
  pub fn new() -> Self {
    Self
  }
}
