use super::flag::Flag;
use super::GitCommand;
use crate::error::Result;
use std::process::{Child, Command, Output, Stdio};

/// <https://git-scm.com/docs/git-status>
pub struct Status {
  command: Command,
  args: Vec<String>,
}

impl Status {
  pub fn new() -> Self {
    Self {
      command: Command::new("git"),
      args: vec!["status".into()],
    }
  }

  /// <https://git-scm.com/docs/git-status#Documentation/git-status.txt---porcelainltversiongt>
  pub fn porcelain(&mut self) -> &mut Self {
    self.args.push(Flag::Porcelain.into());
    self
  }
}

impl GitCommand for Status {
  fn stderr(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stderr(cfg);
    self
  }

  fn stdout(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stdout(cfg);
    self
  }

  fn output(&mut self) -> Result<Output> {
    self.command.args(&self.args).output().map_err(Into::into)
  }

  fn spawn(&mut self) -> Result<Child> {
    self.command.args(&self.args).spawn().map_err(Into::into)
  }
}

impl Default for Status {
  fn default() -> Self {
    Status::new()
  }
}
