use crate::util::MihoCommand;
use anyhow::Result;
use miho_derive::Git;
use std::process::{Child, Command, Output};

/// <https://git-scm.com/docs/git-add>
#[derive(Git)]
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