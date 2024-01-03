use crate::util::MihoCommand;
use anyhow::Result;
use miho_derive::Git;
use std::process::{Child, Command, Output};

/// <https://git-scm.com/docs/git-push>
#[derive(Git)]
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