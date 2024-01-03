mod commands;

use anyhow::Result;
use clap::Parser;
use commands::*;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum MihoCli {
  /// Recursively bump your projects version.
  Bump(BumpCommand),
}

fn main() -> Result<()> {
  let cli = MihoCli::parse();

  match cli {
    MihoCli::Bump(cmd) => cmd.execute(),
  }
}
