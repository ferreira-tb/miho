mod bump;
mod search;
mod update;

pub use bump::Bump;
pub use search::Search;
pub use update::Update;

pub trait Action {
  type Output;

  fn execute(self) -> Self::Output;
}
