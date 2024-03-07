use super::Flag;
use super::Git;
use tokio::process::Command;

/// <https://git-scm.com/docs/git-status>
#[derive(miho_derive::Git)]
pub struct Status {
  command: Command,
  args: Vec<String>,
}

impl Status {
  #[must_use]
  pub fn new() -> Self {
    Self {
      command: Command::new("git"),
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
