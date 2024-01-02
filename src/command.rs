use anyhow::Result;
use std::env;
use std::process::{self, Child, Output, Stdio};

pub trait MihoCommand {
  fn cmd(&mut self) -> &mut process::Command;

  fn spawn(&mut self) -> Result<Child> {
    let child = self.cmd().spawn()?;
    Ok(child)
  }

  fn output(&mut self) -> Result<Output> {
    let output = self.cmd().output()?;
    Ok(output)
  }

  fn stderr(&mut self, cfg: Stdio) -> &mut Self {
    self.cmd().stderr(cfg);
    self
  }

  fn stdout(&mut self, cfg: Stdio) -> &mut Self {
    self.cmd().stdout(cfg);
    self
  }
}

/// Wrap the `Command` of the standard library,
/// executing it in `cmd` if the current OS is Windows.
///
/// This is only useful in some very specific cases.
/// Prefer the standard library version.
pub struct Command {
  cmd: process::Command,
}

impl Command {
  pub fn new<P: AsRef<str>>(program: P) -> Self {
    let program = program.as_ref();

    let mut cmd = match env::consts::OS {
      "windows" => process::Command::new("cmd"),
      _ => process::Command::new(program),
    };

    if env::consts::OS == "windows" {
      cmd.arg("/C").arg(program);
    };

    Self { cmd }
  }

  pub fn arg<A: AsRef<str>>(&mut self, arg: A) -> &mut Command {
    self.cmd.arg(arg.as_ref());
    self
  }

  pub fn args<I, S>(&mut self, args: I) -> &mut Command
  where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
  {
    let iter = args.into_iter();
    let mut args: Vec<String> = vec![];

    for arg in iter {
      let arg = arg.as_ref();
      args.push(arg.to_string());
    }

    self.cmd.args(args);
    self
  }
}

impl MihoCommand for Command {
  fn cmd(&mut self) -> &mut process::Command {
    &mut self.cmd
  }
}
