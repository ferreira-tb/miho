use super::flag::Flag;
use super::Git;
use crate::{git_output, git_spawn};
use std::process::{ExitStatus, Output, Stdio};
use tokio::process::Command;

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

impl Git for Commit {
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
