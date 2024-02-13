use super::Git;
use crate::{git_output, git_spawn, Result};
use std::process::{ExitStatus, Output, Stdio};
use tokio::process::Command;

/// <https://git-scm.com/docs/git-push>
pub struct Push {
  command: Command,
  args: Vec<String>,
}

impl Push {
  #[must_use]
  pub fn new() -> Self {
    Self {
      command: Command::new("git"),
      args: vec!["push".into()],
    }
  }
}

impl Git for Push {
  fn stderr(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stderr(cfg);
    self
  }

  fn stdout(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stdout(cfg);
    self
  }

  async fn spawn(&mut self) -> Result<ExitStatus> {
    git_spawn!(self.command, &self.args)
  }

  async fn output(&mut self) -> Result<Output> {
    git_output!(self.command, &self.args)
  }
}

impl Default for Push {
  fn default() -> Self {
    Push::new()
  }
}
