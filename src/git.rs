mod add;
mod commit;
mod push;

use anyhow::Result;
use std::process::ExitStatus;
use strum::{Display, EnumString};

pub use add::Add;
pub use commit::Commit;
pub use push::Push;

pub trait Git {
  async fn spawn(&mut self) -> Result<ExitStatus>;
}

macro_rules! impl_git {
  ($($name:ident),+) => {
    $(
      impl Git for $name {
        async fn spawn(&mut self) -> Result<ExitStatus> {
          let mut child = self.command.args(&self.args).spawn()?;
          let status = child.wait().await?;
          Ok(status)
        }
      }
    )+
  };
}

impl_git!(Add, Commit, Push);

#[derive(Display, EnumString)]
#[strum(serialize_all = "kebab-case", prefix = "--")]
pub enum Flag {
  All,
  Message,
  NoVerify,
}
