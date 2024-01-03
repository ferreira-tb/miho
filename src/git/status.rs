use super::flag::Flag;
use crate::util::MihoCommand;
use anyhow::Result;
use std::process::{Child, Command, Output};

/// <https://git-scm.com/docs/git-status>
pub struct Status {
  cmd: Command,
  args: Vec<String>,
}

impl Status {
  pub fn new() -> Self {
    Self {
      cmd: Command::new("git"),
      args: vec!["status".into()],
    }
  }

  /// <https://git-scm.com/docs/git-status#Documentation/git-status.txt---porcelainltversiongt>
  pub fn porcelain(&mut self) -> &mut Self {
    self.args.push(Flag::Porcelain.into());
    self
  }
}

impl Default for Status {
  fn default() -> Self {
    Status::new()
  }
}

impl MihoCommand for Status {
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
