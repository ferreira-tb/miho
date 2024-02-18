mod command;
mod prompt;

use anyhow::Result;
use clap::Parser;
use command::{Bump, Command, Run, Update};

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum Cli {
  /// Bump your packages version.
  Bump(Bump),
  /// Run scripts defined in your miho.lua.
  Run(Run),
  /// Update your dependencies.
  Update(Update),
}

#[tokio::main]
async fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli {
    Cli::Bump(cmd) => cmd.execute().await,
    Cli::Run(cmd) => cmd.execute().await,
    Cli::Update(cmd) => cmd.execute().await,
  }
}
