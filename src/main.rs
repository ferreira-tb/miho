use anyhow::Result;
use clap::{Args, Parser};
use colored::*;
use miho::packages;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum MihoCli {
  Bump(BumpArgs),
}

#[derive(Debug, Args)]
struct BumpArgs {
  /// Commit the modified packages.
  #[arg(short = 'c', long, value_name = "MESSAGE")]
  commit: Option<String>,

  /// Do not ask for consent before bumping.
  #[arg(long)]
  no_ask: bool,

  /// Do not push the commit.
  #[arg(long)]
  no_push: bool,

  /// Bypass `pre-commit` and `commit-msg` hooks.
  #[arg(long)]
  no_verify: bool,

  /// Prerelease identifier.
  #[arg(long, value_name = "IDENTIFIER")]
  preid: Option<String>,

  /// Recursively bumps all packages in the monorepo.
  #[arg(short = 'r', long)]
  recursive: bool,

  /// Type of the release.
  #[arg(short = 't', long, value_name = "TYPE", default_value = "patch")]
  release_type: Option<String>,

  /// Describes what to do with the standard I/O stream.
  #[arg(long, default_value = "inherit")]
  stdio: Option<String>,
}

fn main() -> Result<()> {
  let cli = MihoCli::parse();

  match cli {
    MihoCli::Bump(args) => bump(args),
  }
}

fn bump(args: BumpArgs) -> Result<()> {
  let entries = miho::search()?;
  let packages = packages::create_packages(entries)?;

  println!("{:?}", args);

  if packages.is_empty() {
    println!("{}", "No valid package found.".red().bold());
    return Ok(());
  }

  for pkg in packages {
    println!(
      "[ {} ]  {}  =>  newVersion",
      pkg.name.bold(),
      pkg.version.raw().bright_blue()
    );
  }

  Ok(())
}
