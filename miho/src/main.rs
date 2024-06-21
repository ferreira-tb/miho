#![feature(try_blocks)]
#![allow(clippy::module_name_repetitions)]

mod command;
pub(crate) mod git;
mod macros;
pub(crate) mod package;
pub(crate) mod prelude;
pub(crate) mod release;
pub(crate) mod version;

use clap::Parser;
use command::{Bump, Command, Update};
use prelude::*;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum Cli {
  /// Bump your packages version.
  Bump(Bump),
  /// Update your dependencies.
  Update(Update),
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli {
    Cli::Bump(cmd) => cmd.execute().await,
    Cli::Update(cmd) => cmd.execute().await,
  }
}
