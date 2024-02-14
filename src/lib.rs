mod error;
pub mod git;
pub mod package;
mod release;
pub mod version;
mod macros;

pub use error::Error;
pub use release::Release;

pub type Result<T> = std::result::Result<T, Error>;
