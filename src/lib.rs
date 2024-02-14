mod error;
pub mod git;
mod macros;
pub mod package;
mod release;
pub mod version;

pub use error::Error;
pub use release::Release;

pub type Result<T> = std::result::Result<T, Error>;
