mod commands;

use anyhow::Result;
use clap::Parser;
use commands::*;
use miho::{Package, SearchBuilder};

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
    Cli::Bump(mut cmd) => cmd.execute(),
    Cli::Update(cmd) => cmd.execute().await,
  }
}

fn search_packages<P: AsRef<str>>(paths: &[P]) -> anyhow::Result<Vec<Package>> {
  let mut paths: Vec<&str> = paths.iter().map(|g| g.as_ref()).collect();

  let last = paths.pop().unwrap_or(".");
  let mut builder = SearchBuilder::new(last);

  for path in paths {
    builder.add(path);
  }

  Ok(builder.search()?)
}
