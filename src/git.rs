use anyhow::Result;
use std::process::{Command, Stdio};

const ALL: &str = "--all";
const MESSAGE: &str = "--message";
const NO_VERIFY: &str = "--no-verify";

pub struct GitCommit {
  pub message: String,
  pub no_verify: bool,
}

/// https://git-scm.com/docs/git-add
pub fn add(stdio: &str, pathspec: &str) -> Result<()> {
  Command::new("git")
    .args(["add", pathspec])
    .stdout(parse_stdio(stdio))
    .stderr(parse_stdio(stdio))
    .output()?;

  Ok(())
}

/// https://git-scm.com/docs/git-commit
pub fn commit(stdio: &str, flags: GitCommit) -> Result<()> {
  let message = flags.message.as_str();
  let mut args = vec!["commit", MESSAGE, message];

  if flags.no_verify {
    args.push(NO_VERIFY);
  }

  // Should be the last.
  args.push(ALL);

  Command::new("git")
    .args(args)
    .stdout(parse_stdio(stdio))
    .stderr(parse_stdio(stdio))
    .output()?;

  Ok(())
}

/// https://git-scm.com/docs/git-push
pub fn push(stdio: &str) -> Result<()> {
  Command::new("git")
    .arg("push")
    .stdout(parse_stdio(stdio))
    .stderr(parse_stdio(stdio))
    .output()?;

  Ok(())
}

fn parse_stdio(cfg: &str) -> Stdio {
  let cfg = cfg.to_lowercase();
  match cfg.as_str() {
    "null" => Stdio::null(),
    "piped" => Stdio::piped(),
    _ => Stdio::inherit(),
  }
}
