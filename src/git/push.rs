use tokio::process::Command;

/// <https://git-scm.com/docs/git-push>
pub struct Push {
  pub(super) command: Command,
  pub(super) args: Vec<String>,
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
