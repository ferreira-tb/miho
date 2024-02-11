use super::GitCommand;
use std::process::{Child, Command, Output, Stdio};

/// <https://git-scm.com/docs/git-add>
pub struct Add {
  command: Command,
  args: Vec<String>,
}

impl Add {
  pub fn new<T: AsRef<str>>(pathspec: T) -> Self {
    let pathspec = pathspec.as_ref();
    Self {
      command: Command::new("git"),
      args: vec!["add".into(), pathspec.into()],
    }
  }
}

impl GitCommand for Add {
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
