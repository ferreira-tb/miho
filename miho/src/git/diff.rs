use super::Git;
use tokio::process::Command;

/// <https://git-scm.com/docs/git-diff>
#[derive(miho_derive::Git)]
pub struct Diff {
  command: Command,
  args: Vec<String>,
}

impl Diff {
  #[must_use]
  pub fn new() -> Self {
    Self {
      command: Command::new("git"),
      args: vec!["diff".into()],
    }
  }
}

impl Default for Diff {
  fn default() -> Self {
    Diff::new()
  }
}
