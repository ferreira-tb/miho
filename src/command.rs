mod bump;
mod update;

use anyhow::Result;
use strum::{Display, EnumIter, EnumString};

pub use bump::Bump;
pub use update::Update;

pub trait Command {
  async fn execute(self) -> Result<()>;
}

trait Commit: Command {
  async fn commit(&mut self, default_message: &str) -> Result<()>;
}

#[derive(Display, EnumIter, EnumString)]
#[strum(serialize_all = "title_case")]
enum Choice {
  All,
  Some,
  None,
}

enum PromptResult {
  Continue,
  Abort,
}
