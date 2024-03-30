use super::Flag;
use super::Git;
use tokio::process::Command;

/// <https://git-scm.com/docs/git-commit>
#[derive(miho_derive::Git)]
pub struct Commit {
  command: Command,
  args: Vec<String>,
}

impl Commit {
  #[must_use]
  pub fn new<T: AsRef<str>>(message: T) -> Self {
    let message = message.as_ref();
    Self {
      command: Command::new("git"),
      args: vec!["commit".into(), Flag::Message.to_string(), message.into()],
    }
  }

  /// <https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---all>
  pub fn all(&mut self) -> &mut Self {
    self.args.push(Flag::All.to_string());
    self
  }

  /// <https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---no-verify>
  pub fn no_verify(&mut self) -> &mut Self {
    self.args.push(Flag::NoVerify.to_string());
    self
  }
}
