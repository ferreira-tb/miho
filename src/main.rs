#![feature(let_chains, try_blocks)]

mod agent;
mod command;
mod dependency;
mod git;
mod macros;
mod package;
mod prelude;
mod release;
mod version;

use clap::Parser;
use command::{Bump, Command, Config, Update};
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
  let config = Config::load().await?;
  match Cli::parse() {
    Cli::Bump(cmd) => cmd.execute(config).await,
    Cli::Update(cmd) => cmd.execute(config).await,
  }
}
