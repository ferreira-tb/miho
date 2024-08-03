use tokio::process::Command;

/// <https://git-scm.com/docs/git-add>
pub struct Add {
  pub(super) command: Command,
  pub(super) args: Vec<String>,
}

impl Add {
  pub fn new<T: AsRef<str>>(pathspec: T) -> Self {
    let pathspec = pathspec.as_ref();
    Self {
      command: Command::new("git"),
      args: vec!["add".into(), pathspec.into()],
    }
  }
}
