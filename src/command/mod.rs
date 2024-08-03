mod bump;
mod update;

use crate::git::{self, Git};
use crate::prelude::*;
pub use bump::Bump;
use strum::{Display, EnumIter, EnumString};
pub use update::Update;

pub trait Command {
  async fn execute(self) -> Result<()>;
}

trait Commit: Command {
  async fn commit(&mut self, default_message: &str) -> Result<()>;
}

macro_rules! impl_commit {
  ($($name:ident),+) => {
    $(
      impl Commit for $name {
        async fn commit(&mut self, default_message: &str) -> Result<()> {


          if let Some(pathspec) = &self.add {
            git::Add::new(pathspec).spawn().await?;
          }

          let message = if !self.no_ask && self.commit_message.is_none() {
            inquire::Text::new("Commit message: ").prompt_skippable()?
          } else {
            self.commit_message.take()
          };

          let message = match message.as_deref().map(str::trim) {
            Some(m) if !m.is_empty() => m,
            _ => default_message,
          };

          let mut commit = git::Commit::new(message);
          commit.all();

          if self.no_verify {
            commit.no_verify();
          }

          commit.spawn().await?;

          if !self.no_push {
            git::Push::new().spawn().await?;
          }

          Ok(())
        }
      }
    )+
  };
}

impl_commit!(Bump, Update);

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
