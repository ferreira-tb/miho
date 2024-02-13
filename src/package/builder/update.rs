use super::Builder;

pub struct Update;

impl Builder for Update {
  type Output = ();

  fn execute(self) -> crate::Result<Self::Output> {
    unimplemented!()
  }
}

impl Update {
  #[must_use]
  pub fn new() -> Self {
    Self
  }
}
