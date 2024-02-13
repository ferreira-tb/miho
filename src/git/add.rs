use super::Git;
use crate::{git_output, git_spawn};
use std::process::{ExitStatus, Output, Stdio};
use tokio::process::Command;

/// <https://git-scm.com/docs/git-add>
pub struct Add {
  command: Command,
  args: Vec<String>,
}

impl Add {
  #[must_use]
  pub fn new<T: AsRef<str>>(pathspec: T) -> Self {
    let pathspec = pathspec.as_ref();
    Self {
      command: Command::new("git"),
      args: vec!["add".into(), pathspec.into()],
    }
  }
}

impl Git for Add {
  fn stderr(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stderr(cfg);
    self
  }

  fn stdout(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stdout(cfg);
    self
  }

  async fn spawn(&mut self) -> crate::Result<ExitStatus> {
    git_spawn!(self.command, &self.args)
  }

  async fn output(&mut self) -> crate::Result<Output> {
    git_output!(self.command, &self.args)
  }
}
