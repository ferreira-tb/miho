use super::Action;
use crate::Result;

pub struct Update;

impl Action for Update {
  type Output = Result<()>;

  fn execute(self) -> Self::Output {
    unimplemented!()
  }
}

impl Update {
  #[must_use]
  pub fn new() -> Self {
    Self
  }
}
