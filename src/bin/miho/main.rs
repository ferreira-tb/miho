mod command;
mod util;

use anyhow::Result;
use clap::Parser;
use command::{Bump, Update};

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
    Cli::Bump(mut cmd) => cmd.execute().await,
    Cli::Update(cmd) => cmd.execute().await,
  }
}
