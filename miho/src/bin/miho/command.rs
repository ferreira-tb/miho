mod bump;
mod run;
mod update;

use anyhow::Result;
pub use bump::Bump;
pub use run::Run;
use std::fmt;
use std::future::Future;
pub use update::Update;

pub trait Command {
  async fn execute(self) -> Result<()>;
}

trait CommitFromCommand: Command {
  fn commit(&mut self, default_message: &str) -> impl Future<Output = Result<()>> + Send;
}

pub enum Choice {
  All,
  Some,
  None,
}

impl fmt::Display for Choice {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::All => write!(f, "all"),
      Self::Some => write!(f, "some"),
      Self::None => write!(f, "none"),
    }
  }
}
