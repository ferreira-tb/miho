use super::flag::Flag;
use super::GitCommand;
use crate::error::Result;
use std::process::{Child, Command, Output, Stdio};

/// <https://git-scm.com/docs/git-commit>
pub struct Commit {
  command: Command,
  args: Vec<String>,
}

impl Commit {
  pub fn new<T: AsRef<str>>(message: T) -> Self {
    let message = message.as_ref();
    Self {
      command: Command::new("git"),
      args: vec!["commit".into(), Flag::Message.into(), message.into()],
    }
  }

  /// <https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---all>
  pub fn all(&mut self) -> &mut Self {
    self.args.push(Flag::All.into());
    self
  }

  /// <https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---no-verify>
  pub fn no_verify(&mut self) -> &mut Self {
    self.args.push(Flag::NoVerify.into());
    self
  }
}

impl GitCommand for Commit {
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
