mod add;
mod commit;
mod diff;
mod push;
mod status;

pub use add::Add;
use anyhow::Result;
pub use commit::Commit;
pub use diff::Diff;
pub use push::Push;
pub use status::Status;
use std::future::Future;
use std::process::{ExitStatus, Output, Stdio};

pub trait Git {
  fn arg<A: AsRef<str>>(&mut self, arg: A) -> &mut Self;
  fn args<I, A>(&mut self, args: I) -> &mut Self
  where
    I: IntoIterator<Item = A>,
    A: AsRef<str>;

  fn stderr(&mut self, cfg: Stdio) -> &mut Self;
  fn stdout(&mut self, cfg: Stdio) -> &mut Self;

  /// Executes the command as a child process,
  /// returning a future that resolves to [`std::process::ExitStatus`] when the child process completes.
  ///
  /// By default, the stdout/stderr handles are inherited from the parent.
  fn spawn(&mut self) -> impl Future<Output = Result<ExitStatus>> + Send;

  /// Executes the command as a child process,
  /// waiting for it to finish and collecting all of its output.
  ///
  /// This will unconditionally configure the stdout/stderr handles to be pipes,
  /// even if they have been previously configured.
  fn output(&mut self) -> impl Future<Output = Result<Output>> + Send;
}

pub enum Flag {
  All,
  Message,
  NoVerify,
  Porcelain,
}

impl From<Flag> for &str {
  fn from(flag: Flag) -> Self {
    match flag {
      Flag::All => "--all",
      Flag::Message => "--message",
      Flag::NoVerify => "--no-verify",
      Flag::Porcelain => "--porcelain",
    }
  }
}

impl From<Flag> for String {
  fn from(flag: Flag) -> Self {
    let raw: &str = flag.into();
    String::from(raw)
  }
}

macro_rules! bail_on_error {
  ($result:expr) => {
    if !$result.status.success() {
      let stderr = String::from_utf8_lossy(&$result.stderr).into_owned();
      anyhow::bail!("{}", stderr);
    }
  };
}

pub async fn is_dirty() -> Result<bool> {
  let diff = Diff::new().output().await?;
  bail_on_error!(diff);

  if !diff.stdout.is_empty() {
    return Ok(true);
  }

  let output = Status::new().porcelain().output().await?;
  bail_on_error!(output);

  let is_empty = output.stdout.is_empty();

  Ok(!is_empty)
}
