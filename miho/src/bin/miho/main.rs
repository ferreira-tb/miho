mod commands;

use anyhow::Result;
use clap::Parser;
use commands::*;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum Cli {
  /// Recursively bump your packages version.
  Bump(Bump),

  Update(Update),
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli {
    Cli::Bump(mut cmd) => cmd.execute(),
    Cli::Update(mut cmd) => cmd.execute(),
  }
}
