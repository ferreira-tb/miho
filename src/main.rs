use anyhow::Result;
use clap::{Args, Parser};
use colored::*;
use inquire::{Confirm, MultiSelect, Select};
use miho::bump;
use miho::git::{self, GitCommit};
use miho::packages::{self, Package};
use miho::semver::{self, ReleaseType};

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum MihoCli {
  Bump(BumpCommand),
}

#[derive(Debug, Args)]
struct BumpCommand {
  /// Include untracked files with `git add <PATHSPEC>`.
  #[arg(short = 'a', long, value_name = "PATHSPEC")]
  add: Option<String>,

  /// Commit the modified packages.
  #[arg(short = 'm', long, value_name = "MESSAGE")]
  commit_message: Option<String>,

  /// Do not ask for consent before bumping.
  #[arg(long)]
  no_ask: bool,

  /// Do not commit the modified packages.
  #[arg(long)]
  no_commit: bool,

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

    let release_type = self.release_type()?;
    let pre_id = self.pre_id();

    for package in &packages {
      let new_version = package.version.inc(&release_type, pre_id)?;

      println!(
        "[ {} ]  {}  =>  {}",
        package.name.bold(),
        package.version.raw().bright_blue(),
        new_version.raw().bright_green()
      );
    }

    if !self.no_ask {
      self.prompt(packages, release_type)?;
    } else {
      bump::bump(packages, release_type, pre_id)?;
    }

    if !self.no_commit {
      let stdio = self.stdio();

      if let Some(add) = &self.add {
        git::add(&stdio, add)?;
      }

      let message = match &self.commit_message {
        Some(m) => m.clone(),
        None => String::from("chore: bump version"),
      };

      let commit_flags = GitCommit {
        message,
        no_verify: self.no_verify
      };
  
      git::commit(&stdio, commit_flags)?;
  
      if !self.no_push {
        git::push(&stdio)?;
      }
    }
    
    Ok(())
  }

  fn prompt(&self, packages: Vec<Package>, release_type: ReleaseType) -> Result<()> {
    if packages.len() == 1 {
      let name = &packages.first().unwrap().name;
      let message = format!("Bump {}?", name);
      let response = Confirm::new(&message).with_default(true).prompt()?;

      if response {
        bump::bump(packages, release_type, self.pre_id())?;
      }

      Ok(())
    } else {
      let options = vec!["All", "Some", "None"];
      let response = Select::new("Select what to bump.", options).prompt()?;

      match response {
        "All" => bump::bump(packages, release_type, self.pre_id()),
        "Some" => {
          let message = "Select the packages to bump.";
          let packages = MultiSelect::new(message, packages).prompt()?;
          bump::bump(packages, release_type, self.pre_id())
        }
        _ => Ok(()),
      }
    }
  }

  fn pre_id(&self) -> Option<&str> {
    self.pre_id.as_deref()
  }

  fn release_type(&self) -> Result<ReleaseType> {
    let rt = match &self.release_type {
      Some(rt) => semver::to_release_type(rt)?,
      None => semver::to_release_type("patch")?,
    };

    Ok(rt)
  }

  fn stdio(&self) -> String {
    match &self.stdio {
      Some(m) => m.clone(),
      None => String::from("inherit"),
    }
  }
}

fn main() -> Result<()> {
  let cli = MihoCli::parse();

  match cli {
    MihoCli::Bump(cmd) => cmd.execute(),
  }
}
