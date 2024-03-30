use super::Git;
use crate::prelude::*;

/// <https://git-scm.com/docs/git-add>
#[derive(miho_derive::Git)]
pub struct Add {
  command: Command,
  args: Vec<String>,
}

impl Add {
  #[must_use]
  pub fn new<T: AsRef<str>>(pathspec: T) -> Self {
    let pathspec = pathspec.as_ref();
    Self {
      command: Command::new("git"),
      args: vec!["add".into(), pathspec.into()],
    }
  }
}
