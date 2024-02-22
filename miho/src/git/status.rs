use super::Flag;
use super::Git;
use anyhow::{bail, Result};
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

  pub async fn is_dirty() -> Result<bool> {
    let output = Self::new().output().await?;

    if !output.status.success() {
      let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
      bail!("{stderr}");
    }

    let is_empty = output.stdout.is_empty();

    Ok(!is_empty)
  }
}

impl Default for Status {
  fn default() -> Self {
    Status::new()
  }
}
