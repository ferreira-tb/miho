use miho_derive::GitCommand;
use std::process::Command;

/// <https://git-scm.com/docs/git-add>
#[derive(GitCommand)]
pub struct Add {
  cmd: Command,
  args: Vec<String>,
}

impl Add {
  pub fn new<T: AsRef<str>>(pathspec: T) -> Self {
    let pathspec = pathspec.as_ref();
    Self {
      cmd: Command::new("git"),
      args: vec!["add".into(), pathspec.into()],
    }
  }
}
