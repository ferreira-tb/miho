use super::flag::Flag;
use crate::util::MihoCommand;
use anyhow::Result;
use miho_derive::Git;
use std::process::{Child, Command, Output};

/// <https://git-scm.com/docs/git-status>
#[derive(Git)]
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
