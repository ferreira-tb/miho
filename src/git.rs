mod commit;
mod flag;

use anyhow::Result;
pub use flag::Flag;
pub use commit::Commit;
use std::process::{Command, Stdio};

/// <https://git-scm.com/docs/git-add>
pub fn add<P: AsRef<str>>(pathspec: P) -> Result<()> {
  let pathspec = pathspec.as_ref();

  Command::new("git")
    .args(["add", pathspec])
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()?;

  Ok(())
}

/// <https://git-scm.com/docs/git-push>
pub fn push() -> Result<()> {
  Command::new("git")
    .arg("push")
    .stdout(Stdio::inherit())
    .stderr(Stdio::inherit())
    .output()?;

  Ok(())
}

/// <https://git-scm.com/docs/git-status>
pub fn is_dirty() -> Result<bool> {
  let output = Command::new("git")
    .args(["status", Flag::Porcelain.into()])
    .output()?;

  if output.stdout.is_empty() {
    Ok(false)
  } else {
    Ok(true)
  }
}
