mod add;
mod commit;
mod push;

use crate::prelude::*;
pub use add::Add;
pub use commit::Commit;
pub use push::Push;
use std::process::{ExitStatus, Output, Stdio};
use strum::{Display, EnumString};

pub trait Git {
  fn arg<A: AsRef<str>>(&mut self, arg: A) -> &mut Self;
  fn args<I, A>(&mut self, args: I) -> &mut Self
  where
    I: IntoIterator<Item = A>,
    A: AsRef<str>;

  fn stderr(&mut self, cfg: Stdio) -> &mut Self;
  fn stdout(&mut self, cfg: Stdio) -> &mut Self;

  async fn spawn(&mut self) -> Result<ExitStatus>;
  async fn output(&mut self) -> Result<Output>;
}

#[derive(Display, EnumString)]
#[strum(serialize_all = "kebab-case", prefix = "--")]
pub enum Flag {
  All,
  Message,
  NoVerify,
}
