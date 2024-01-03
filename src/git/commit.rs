use super::flag::Flag;
use crate::MihoCommand;
use anyhow::Result;
use std::process::{Child, Command, Output};

/// <https://git-scm.com/docs/git-commit>
pub struct Commit {
  cmd: Command,
  args: Vec<String>,
}

impl Commit {
  pub fn new<T: AsRef<str>>(message: T) -> Self {
    let message = message.as_ref();
    Self {
      cmd: Command::new("git"),
      args: vec!["commit".into(), Flag::Message.into(), message.to_string()],
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

impl MihoCommand for Commit {
  fn cmd(&mut self) -> &mut Command {
    &mut self.cmd
  }

  fn output(&mut self) -> Result<Output> {
    let args = self.args.as_slice();
    let output = self.cmd.args(args).output()?;
    Ok(output)
  }

  fn spawn(&mut self) -> Result<Child> {
    let args = self.args.as_slice();
    let child = self.cmd.args(args).spawn()?;
    Ok(child)
  }
}
