mod stdio;

use anyhow::Result;
use std::env;
use std::process::{self, Output};
pub use stdio::Stdio;

pub struct Command {
  cmd: process::Command,
}

impl Command {
  pub fn new<S: AsRef<str>>(program: S) -> Self {
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

  pub fn arg<S: AsRef<str>>(&mut self, arg: S) -> &mut Command {
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

  pub fn raw(self) -> process::Command {
    self.cmd
  }

  pub fn stdio(&mut self, cfg: Stdio) -> &mut Command {
    self.cmd.stderr(cfg.as_std_stdio());
    self.cmd.stdout(cfg.as_std_stdio());
    self
  }

  pub fn output(&mut self) -> Result<Output> {
    let output = self.cmd.output()?;
    Ok(output)
  }
}
