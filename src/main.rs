use anyhow::Result;
use clap::{Args, Parser};
use colored::*;
use inquire::{Confirm, Select};
use miho::{packages, semver};
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum MihoCli {
  Bump(BumpCommand),
}

#[derive(Debug, Args)]
struct BumpCommand {
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
  #[arg(short = 'i', long, value_name = "IDENTIFIER")]
  pre_id: Option<String>,

  /// Recursively bumps all packages in the monorepo.
  #[arg(short = 'r', long)]
  recursive: bool,

  /// Type of the release.
  #[arg(short = 't', long, value_name = "TYPE")]
  release_type: Option<String>,

  /// Describes what to do with the standard I/O stream.
  #[arg(long, default_value = "inherit")]
  stdio: Option<String>,
}

impl BumpCommand {
  fn execute(&self) -> Result<()> {
    let entries = miho::search()?;
    let packages = packages::create_packages(entries)?;

    if packages.is_empty() {
      println!("{}", "No valid package found.".bold().red());
      return Ok(());
    }

    let pre_id = self.pre_id.as_deref();
    let release_type = match &self.release_type {
      Some(rt) => semver::to_release_type(rt)?,
      None => semver::to_release_type("patch")?,
    };

    let mut new_version_map: HashMap<u32, semver::Version> = HashMap::new();

    for package in &packages {
      let new_version = package.version.inc(&release_type, pre_id)?;
      let new_version_raw = new_version.raw();
      new_version_map.insert(package.id, new_version);

      println!(
        "[ {} ]  {}  =>  {}",
        package.name.bold(),
        package.version.raw().bright_blue(),
        new_version_raw.bright_green()
      );
    }

    if !self.no_ask {
      self.prompt(&packages)?;
    }

    Ok(())
  }

  fn prompt(&self, packages: &Vec<packages::Package>) -> Result<()> {
    if packages.len() == 1 {
      let name = &packages.first().unwrap().name;
      let message = format!("Bump {}?", name);
      let ans = Confirm::new(&message).with_default(true).prompt()?;

      match ans {
        true => todo!(),
        false => Ok(()),
      }
    } else {
      let options = vec!["All", "Some", "None"];
      let ans = Select::new("Select what to bump.", options).prompt()?;

      match ans {
        "All" => todo!(),
        "Some" => todo!(),
        _ => Ok(()),
      }
    }
  }
}

fn main() -> Result<()> {
  let cli = MihoCli::parse();

  match cli {
    MihoCli::Bump(cmd) => cmd.execute(),
  }
}
