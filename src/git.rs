mod add;
mod commit;
mod flag;
mod push;
mod status;

use crate::bail;
use crate::error::{Error, Result};
pub use add::Add;
pub use commit::Commit;
pub use flag::Flag;
pub use push::Push;
pub use status::Status;
use std::process::{Child, Output, Stdio};

pub trait GitCommand {
  fn stderr(&mut self, cfg: Stdio) -> &mut Self;
  fn stdout(&mut self, cfg: Stdio) -> &mut Self;
  fn output(&mut self) -> Result<Output>;
  fn spawn(&mut self) -> Result<Child>;
}

/// Determines whether there are uncommitted changes.
pub fn is_dirty() -> Result<bool> {
  let output = Status::new()
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .output()?;

  if !output.status.success() {
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    bail!(Error::Git { reason: stderr });
  }

  let is_empty = output.stdout.is_empty();

  Ok(!is_empty)
}
