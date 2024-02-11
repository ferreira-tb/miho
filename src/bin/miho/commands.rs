mod bump;
mod update;

pub use bump::Bump;
pub use update::Update;

pub trait Command {
  fn execute(&mut self) -> anyhow::Result<()>;
}
