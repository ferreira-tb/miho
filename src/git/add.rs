use crate::MihoCommand;
use anyhow::Result;
use std::process::{Child, Command, Output};

/// <https://git-scm.com/docs/git-add>
pub struct Add {
  cmd: Command,
  args: Vec<String>,
}

impl Add {
  pub fn new<T: AsRef<str>>(pathspec: T) -> Self {
    let pathspec = pathspec.as_ref();
    Self {
      cmd: Command::new("git"),
      args: vec!["add".into(), pathspec.into()],
    }
  }
}

impl MihoCommand for Add {
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
