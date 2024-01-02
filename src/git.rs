mod add;
mod commit;
mod flag;
mod push;
mod status;

use crate::MihoCommand;
pub use add::Add;
use anyhow::Result;
pub use commit::Commit;
pub use flag::Flag;
pub use push::Push;
pub use status::Status;
use std::process::Stdio;

/// Determine whether there are uncommitted changes.
pub fn is_dirty() -> Result<bool> {
  let output = Status::new().stdout(Stdio::piped()).output()?;

  if output.stdout.is_empty() {
    Ok(false)
  } else {
    Ok(true)
  }
}
