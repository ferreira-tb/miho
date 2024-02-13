mod bump;
mod search;
mod update;

use crate::Result;
pub use bump::Bump;
pub use search::Search;
pub use update::Update;

pub trait Builder {
  type Output;

  fn execute(self) -> Result<Self::Output>;
}
