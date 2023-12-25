use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct MihoCli {
  /// Commit all modififed files, not only the packages.
  #[arg(short = 'a', long, default_value_t = true)]
  all: bool,

  /// Determines whether Miho should ask for consent.
  #[arg(long, default_value_t = true)]
  ask: bool,

  /// Commit the modified packages.
  #[arg(short = 'c', long, default_value = "chore: bump version")]
  commit: String,

  /// Skip all jobs.
  #[arg(long)]
  dry_run: bool,
}

fn main() -> Result<()> {
  let cli = MihoCli::parse();
  println!("{:?}", cli);



  Ok(())
}
