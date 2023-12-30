mod stdio;

use anyhow::Result;
use std::env;
use std::process::{self, Output};
pub use stdio::MihoStdio;

pub struct Command {
  cmd: process::Command,
}

impl Command {
  pub fn new<S: AsRef<str>>(program: S) -> Self {
    let program = program.as_ref();

    let mut cmd: process::Command = match env::consts::OS {
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

  pub fn args(&mut self, args: Vec<&str>) -> &mut Command {
    self.cmd.args(args);
    self
  }

  pub fn raw(self) -> process::Command {
    self.cmd
  }

  pub fn stdio<M: AsRef<MihoStdio>>(&mut self, cfg: M) -> &mut Command {
    let cfg = cfg.as_ref();
    self.cmd.stderr(cfg.as_stdio());
    self.cmd.stdout(cfg.as_stdio());
    self
  }

  pub fn output(&mut self) -> Result<Output> {
    let output = self.cmd.output()?;
    Ok(output)
  }
}
