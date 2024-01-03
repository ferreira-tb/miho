use anyhow::Result;
use clap::{Args, Parser};
use colored::*;
use inquire::{Confirm, MultiSelect, Select};
use miho::git::{Add, Commit, Push};
use miho::package::{PackageParser, SearchBuilder, Transaction};
use miho::semver::ReleaseType;
use miho::MihoCommand;
use std::process::Stdio;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum MihoCli {
  Bump(BumpCommand),
}

#[derive(Debug, Args)]
struct BumpCommand {
  /// Type of the release.
  release_type: Option<String>,

  /// Include untracked files with `git add <PATHSPEC>`.
  #[arg(short = 'a', long, value_name = "PATHSPEC")]
  add: Option<String>,

  /// Commit the modified packages.
  #[arg(short = 'm', long, value_name = "MESSAGE")]
  commit_message: Option<String>,

  /// Where to search for packages.
  #[arg(short = 'i', long = "include", value_name = "GLOB")]
  globs: Option<Vec<String>>,

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
  #[arg(long, value_name = "IDENTIFIER")]
  pre_id: Option<String>,

  /// Describes what to do with the standard I/O stream.
  #[arg(short = 's', long, default_value = "inherit")]
  stdio: Option<String>,
}

impl BumpCommand {
  fn execute(&self) -> Result<()> {
    let entries = match &self.globs {
      Some(globs) if !globs.is_empty() => {
        let mut globs: Vec<&str> = globs.iter().map(|g| g.as_str()).collect();
        let last = globs.pop().unwrap_or(".");
        let mut builder = SearchBuilder::new(last);
        for glob in globs {
          builder.add(glob);
        }

        builder.search()?
      }
      _ => {
        let builder = SearchBuilder::new(".");
        builder.search()?
      }
    };

    let pre_id = self.pre_id.as_deref();
    let release_type = match self.release_type.as_deref() {
      Some(rt) => rt.try_into()?,
      None => ReleaseType::Patch,
    };

    let mut parser = PackageParser::new(entries);
    parser.release(&release_type);

    if let Some(id) = pre_id {
      parser.pre_id(id);
    }

    let packages = parser.parse()?;
    if packages.is_empty() {
      println!("{}", "No valid package found.".bold().red());
      return Ok(());
    }

    for package in &packages {
      let new_version = package.version.inc(&release_type, pre_id)?;

      println!(
        "[ {} ]  {}  =>  {}",
        package.name.bold(),
        package.version.raw().bright_blue(),
        new_version.raw().bright_green()
      );
    }

    let transaction = Transaction::new(packages);

    if !self.no_ask {
      let should_continue = self.prompt(transaction)?;
      if !should_continue {
        return Ok(());
      }
    } else {
      transaction.commit()?;
    }

    if !self.no_commit {
      let stdio = match &self.stdio {
        Some(m) => m.as_str(),
        None => "inherit",
      };

      if let Some(pathspec) = &self.add {
        Add::new(pathspec)
          .stderr(stdio.to_stdio())
          .stdout(stdio.to_stdio())
          .output()?;
      }

      let message = match &self.commit_message {
        Some(m) => m,
        None => "chore: bump version",
      };

      let mut commit = Commit::new(message);
      commit.stderr(stdio.to_stdio()).stdout(stdio.to_stdio());

      if self.no_verify {
        commit.no_verify();
      }

      commit.all().output()?;

      if !self.no_push {
        Push::new()
          .stderr(stdio.to_stdio())
          .stdout(stdio.to_stdio())
          .output()?;
      }
    }

    Ok(())
  }

  fn prompt(&self, mut transaction: Transaction) -> Result<bool> {
    if transaction.packages.len() == 1 {
      let name = &transaction.packages.first().unwrap().name;
      let message = format!("Bump {}?", name);
      let response = Confirm::new(&message).with_default(true).prompt()?;

      if response {
        transaction.commit()?;
        Ok(true)
      } else {
        Ok(false)
      }
    } else {
      let options = vec!["All", "Some", "None"];
      let response = Select::new("Select what to bump.", options).prompt()?;

      match response {
        "All" => {
          transaction.commit()?;
          Ok(true)
        }
        "Some" => {
          let message = "Select the packages to bump.";
          let packages = transaction.packages;
          transaction.packages = MultiSelect::new(message, packages).prompt()?;
          transaction.commit()?;
          Ok(true)
        }
        _ => Ok(false),
      }
    }
  }
}

pub trait StdioStr<T: AsRef<str>> {
  fn to_stdio(&self) -> Stdio;
}

impl<T: AsRef<str>> StdioStr<T> for T {
  fn to_stdio(&self) -> Stdio {
    let value = self.as_ref();
    let value = value.trim().to_lowercase();
    match value.as_str() {
      "null" => Stdio::null(),
      "pipe" | "piped" => Stdio::piped(),
      _ => Stdio::inherit(),
    }
  }
}

fn main() -> Result<()> {
  let cli = MihoCli::parse();

  match cli {
    MihoCli::Bump(cmd) => cmd.execute(),
  }
}
