use miho_derive::GitCommand;
use std::process::Command;

/// <https://git-scm.com/docs/git-push>
#[derive(GitCommand)]
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
