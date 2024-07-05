mod add;
mod commit;
mod push;

use crate::prelude::*;
pub use add::Add;
pub use commit::Commit;
pub use push::Push;
use std::process::ExitStatus;
use strum::{Display, EnumString};

pub trait Git {
  async fn spawn(&mut self) -> Result<ExitStatus>;
}

#[derive(Display, EnumString)]
#[strum(serialize_all = "kebab-case", prefix = "--")]
pub enum Flag {
  All,
  Message,
  NoVerify,
}
