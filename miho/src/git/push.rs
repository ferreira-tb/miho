use super::Git;
use tokio::process::Command;

/// <https://git-scm.com/docs/git-push>
#[derive(miho_derive::Git)]
pub struct Push {
  command: Command,
  args: Vec<String>,
}

impl Push {
  pub fn new() -> Self {
    Self {
      command: Command::new("git"),
      args: vec!["push".into()],
    }
  }
}

impl Default for Push {
  fn default() -> Self {
    Push::new()
  }
}
