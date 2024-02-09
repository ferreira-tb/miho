mod bump;
mod update;

use anyhow::Result;
pub use bump::Bump;
pub use update::Update;

pub trait Command {
  fn execute(&mut self) -> Result<()>;
}
