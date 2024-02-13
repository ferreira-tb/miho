mod add;
mod commit;
mod flag;
mod push;
mod status;

use crate::bail;
use crate::error::Error;
pub use add::Add;
pub use commit::Commit;
pub use flag::Flag;
pub use push::Push;
pub use status::Status;
use std::future::Future;
use std::process::{ExitStatus, Output, Stdio};

pub trait Git {
  fn stderr(&mut self, cfg: Stdio) -> &mut Self;
  fn stdout(&mut self, cfg: Stdio) -> &mut Self;

  /// Executes the command as a child process,
  /// returning a future that resolves to an `ExitStatus` when the child process completes.
  ///
  /// By default, the stdout/stderr handles are inherited from the parent.
  fn spawn(&mut self) -> impl Future<Output = crate::Result<ExitStatus>> + Send;

  /// Executes the command as a child process,
  /// waiting for it to finish and collecting all of its output.
  ///
  /// This will unconditionally configure the stdout/stderr handles to be pipes,
  /// even if they have been previously configured.
  fn output(&mut self) -> impl Future<Output = crate::Result<Output>> + Send;
}

#[doc(hidden)]
#[macro_export]
macro_rules! git_spawn {
  ($command:expr, $args:expr) => {{
    type Child = $crate::Result<tokio::process::Child>;
    let child: Child = $command.args($args).spawn().map_err(Into::into);
    child?.wait().await.map_err(Into::into)
  }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! git_output {
  ($command:expr, $args:expr) => {{
    $command.args($args).output().await.map_err(Into::into)
  }};
}

/// Determines whether there are uncommitted changes.
pub async fn is_dirty() -> crate::Result<bool> {
  let output = Status::new().output().await?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    bail!(Error::Git { reason: stderr });
  }

  let is_empty = output.stdout.is_empty();

  Ok(!is_empty)
}
