use super::Flag;
use tokio::process::Command;

/// <https://git-scm.com/docs/git-commit>
pub struct Commit {
  pub(super) command: Command,
  pub(super) args: Vec<String>,
}

impl Commit {
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
