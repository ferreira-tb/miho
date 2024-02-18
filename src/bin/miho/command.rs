mod bump;
mod run;
mod update;

use anyhow::Result;
pub use bump::Bump;
pub use run::Run;
pub use update::Update;

pub trait Command {
  async fn execute(self) -> Result<()>;
}
