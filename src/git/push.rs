use super::GitCommand;
use std::process::{Child, Command, Output, Stdio};

/// <https://git-scm.com/docs/git-push>
pub struct Push {
  command: Command,
  args: Vec<String>,
}

impl Push {
  pub fn new() -> Self {
    Self {
      command: Command::new("git"),
      args: vec!["push".into()],
    }
  }
}

impl GitCommand for Push {
  fn stderr(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stderr(cfg);
    self
  }

  fn stdout(&mut self, cfg: Stdio) -> &mut Self {
    self.command.stdout(cfg);
    self
  }

  fn output(&mut self) -> crate::Result<Output> {
    self.command.args(&self.args).output().map_err(Into::into)
  }

  fn spawn(&mut self) -> crate::Result<Child> {
    self.command.args(&self.args).spawn().map_err(Into::into)
  }
}

impl Default for Push {
  fn default() -> Self {
    Push::new()
  }
}
