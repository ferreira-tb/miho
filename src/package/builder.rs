mod bump;
mod search;
mod update;

pub use bump::Bump;
pub use search::Search;
pub use update::Update;

pub trait Builder {
  type Output;

  fn execute(self) -> Self::Output;
}
