mod bump;
mod run;
mod update;

use anyhow::Result;
pub use bump::Bump;
pub use run::Run;
use std::fmt;
pub use update::Update;

pub trait Command {
  async fn execute(self) -> Result<()>;
}

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
