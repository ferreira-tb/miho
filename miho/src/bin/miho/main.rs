mod commands;

use anyhow::Result;
use clap::Parser;
use commands::*;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum Cli {
  /// Recursively bump your packages version.
  Bump(BumpCommand),
}

fn main() -> Result<()> {
  let cli = Cli::parse();

  match cli {
    Cli::Bump(cmd) => cmd.execute(),
  }
}
