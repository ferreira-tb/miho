mod bump;
mod update;

use crate::prelude::*;
pub use bump::Bump;
use strum::{Display, EnumIter, EnumString};
pub use update::Update;

pub trait Command {
  async fn execute(self) -> Result<()>;
}

pub trait Commit: Command {
  async fn commit(&mut self, default_message: &str) -> Result<()>;
}

#[derive(Display, EnumIter, EnumString)]
#[strum(serialize_all = "title_case")]
pub enum Choice {
  All,
  Some,
  None,
}
