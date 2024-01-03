use crate::util::MihoCommand;
use anyhow::Result;
use std::process::{Child, Command, Output};

/// <https://git-scm.com/docs/git-push>
pub struct Push {
  cmd: Command,
  args: Vec<String>,
}

impl Push {
  pub fn new() -> Self {
    Self {
      cmd: Command::new("git"),
      args: vec!["push".into()],
    }
  }
}

impl Default for Push {
  fn default() -> Self {
    Push::new()
  }
}

impl MihoCommand for Push {
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
