use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum MihoCli {
  Bump {
    /// Commit the modified packages.
    #[arg(short = 'c', long, value_name = "MESSAGE")]
    commit: Option<String>,

    /// Package names to filter.
    #[arg(short = 'f', long, value_name = "PACKAGE")]
    filter: Option<Vec<String>>,

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
    #[arg(long, default_value = "alpha", value_name = "IDENTIFIER")]
    preid: Option<String>,

    /// Recursively bumps all packages in the monorepo.
    #[arg(short = 'r', long)]
    recursive: bool,

    /// Describes what to do with the standard I/O stream.
    #[arg(long, default_value = "inherit")]
    stdio: Option<String>,
  },
}

fn main() -> Result<()> {
  let cli = MihoCli::parse();
  println!("{:?}", cli);

  let entries = miho::search()?;

  for entry in &entries {
    println!("{}", entry.display());
  }

  let packages = miho::packages::create_packages(entries)?;
  println!("{:?}", packages);

  Ok(())
}
