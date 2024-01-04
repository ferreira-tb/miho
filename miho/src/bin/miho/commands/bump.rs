use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use inquire::{Confirm, MultiSelect, Select};
use miho::git::{Add, Commit, Push};
use miho::package::{Package, SearchBuilder};
use miho::versioning::semver::ReleaseType;
use std::process::Stdio;

#[derive(Debug, Args)]
pub struct BumpCommand {
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
  #[arg(short = 'n', long)]
  no_verify: bool,

  /// Prerelease identifier.
  #[arg(long, value_name = "IDENTIFIER")]
  pre_id: Option<String>,

  /// Describes what to do with the standard I/O stream.
  #[arg(short = 's', long, default_value = "inherit")]
  stdio: Option<String>,
}

impl BumpCommand {
  pub fn execute(&self) -> Result<()> {
    let packages = match &self.globs {
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

    if packages.is_empty() {
      println!("{}", "No valid package found.".bold().red());
      return Ok(());
    }

    let pre_id = self.pre_id.as_deref();
    let release_type = match self.release_type.as_deref() {
      Some(rt) => rt.try_into()?,
      None => ReleaseType::Patch,
    };

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
      let should_continue = self.prompt(packages, release_type)?;
      if !should_continue {
        return Ok(());
      }
    } else {
      self.bump_all(packages, release_type)?;
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
          .output()
          .with_context(|| "failed to update git index")?;
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

      commit
        .all()
        .output()
        .with_context(|| "failed to commit packages")?;

      if !self.no_push {
        Push::new()
          .stderr(stdio.to_stdio())
          .stdout(stdio.to_stdio())
          .output()
          .with_context(|| "failed to push commit")?;
      }
    }

    Ok(())
  }

  fn prompt(&self, packages: Vec<Package>, rt: ReleaseType) -> Result<bool> {
    let pre_id = self.pre_id.as_deref();

    if packages.len() == 1 {
      let package = packages.first().unwrap();
      let message = format!("Bump {}?", package.name);
      let response = Confirm::new(&message).with_default(true).prompt()?;

      if response {
        package.bump(&rt, pre_id)?;
        Ok(true)
      } else {
        Ok(false)
      }
    } else {
      let options = vec!["All", "Some", "None"];
      let response = Select::new("Select what to bump.", options).prompt()?;

      match response {
        "All" => {
          self.bump_all(packages, rt)?;
          Ok(true)
        }
        "Some" => {
          let message = "Select the packages to bump.";
          let packages = MultiSelect::new(message, packages).prompt()?;
          self.bump_all(packages, rt)?;
          Ok(true)
        }
        _ => Ok(false),
      }
    }
  }

  fn bump_all(&self, packages: Vec<Package>, rt: ReleaseType) -> Result<()> {
    let pre_id = self.pre_id.as_deref();
    for package in packages {
      package
        .bump(&rt, pre_id)
        .with_context(|| "failed to bump all packages")?;
    }

    Ok(())
  }
}

trait StdioStr<T: AsRef<str>> {
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
