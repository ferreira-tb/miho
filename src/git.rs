use crate::stdio::MihoStdio;
use anyhow::Result;
use std::process::Command;

const ALL: &str = "--all";
const MESSAGE: &str = "--message";
const NO_VERIFY: &str = "--no-verify";

pub struct GitCommit {
  pub message: String,
  pub no_verify: bool,
}

/// https://git-scm.com/docs/git-add
pub fn add(stdio: MihoStdio, pathspec: &str) -> Result<()> {
  Command::new("git")
    .args(["add", pathspec])
    .stdout(stdio.as_stdio())
    .stderr(stdio.as_stdio())
    .output()?;

  Ok(())
}

/// https://git-scm.com/docs/git-commit
pub fn commit(stdio: MihoStdio, flags: GitCommit) -> Result<()> {
  let message = flags.message.as_str();
  let mut args = vec!["commit", MESSAGE, message];

  if flags.no_verify {
    args.push(NO_VERIFY);
  }

  // Should be the last.
  args.push(ALL);

  Command::new("git")
    .args(args)
    .stdout(stdio.as_stdio())
    .stderr(stdio.as_stdio())
    .output()?;

  Ok(())
}

/// https://git-scm.com/docs/git-push
pub fn push(stdio: MihoStdio) -> Result<()> {
  Command::new("git")
    .arg("push")
    .stdout(stdio.as_stdio())
    .stderr(stdio.as_stdio())
    .output()?;

  Ok(())
}

pub fn is_dirty() -> Result<bool> {
  let output = Command::new("git")
    .args(["status", "--porcelain"])
    .output()?;

  println!("{}", output.stdout.len());
  if output.stdout.is_empty() {
    Ok(true)
  } else {
    Ok(false)
  }
}
