mod add;
mod commit;
mod push;
mod status;

pub use add::Add;
use anyhow::Result;
pub use commit::Commit;
pub use push::Push;
pub use status::Status;
use std::future::Future;
use std::process::{ExitStatus, Output, Stdio};

pub trait Git {
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
