use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use inquire::{Confirm, MultiSelect, Select};
use miho::git::{Add, Commit, Push};
use miho::package::{Package, SearchBuilder};
use miho::release::Release;
use semver::Prerelease;
use std::process::Stdio;

#[derive(Debug, Args)]
pub struct BumpCommand {
  /// Type of the release.
  release: Option<String>,

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
  #[arg(short = 'k', long)]
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
  #[arg(short = 'p', long, value_name = "IDENTIFIER")]
  pre: Option<String>,

  /// Build metadata.
  #[arg(short = 'b', long, value_name = "METADATA")]
  build: Option<String>,
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

    let pre = self.pre.as_deref();
    let release = match self.release.as_deref() {
      Some(rt) => rt.try_into()?,
      None => Release::Patch,
    };

    for package in &packages {
      let new_version = match pre {
        Some(pre) => release.increment_pre(&package.version, Prerelease::new(pre)?),
        None => release.increment(&package.version),
      };

      println!(
        "[ {} ]  {}  =>  {}",
        package.name.bold(),
        package.version.to_string().bright_blue(),
        new_version.to_string().bright_green()
      );
    }

    if !self.no_ask {
      let should_continue = self.prompt(packages, release)?;
      if !should_continue {
        return Ok(());
      }
    } else {
      self.bump_all(packages, release)?;
    }

    if !self.no_commit {
      if let Some(pathspec) = &self.add {
        Add::new(pathspec)
          .stderr(Stdio::inherit())
          .stdout(Stdio::inherit())
          .output()
          .with_context(|| "failed to update git index")?;
      }

      let message = match &self.commit_message {
        Some(m) => m.trim(),
        None => "chore: bump version",
      };

      let mut commit = Commit::new(message);
      commit.stderr(Stdio::inherit()).stdout(Stdio::inherit());

      if self.no_verify {
        commit.no_verify();
      }

      commit
        .all()
        .output()
        .with_context(|| "failed to commit packages")?;

      if !self.no_push {
        Push::new()
          .stderr(Stdio::inherit())
          .stdout(Stdio::inherit())
          .output()
          .with_context(|| "failed to push commit")?;
      }
    }

    Ok(())
  }

  fn prompt(&self, mut packages: Vec<Package>, release: Release) -> Result<bool> {
    if packages.len() == 1 {
      let package = packages.swap_remove(0);
      return self.prompt_single(package, release);
    } else {
      let options = vec!["All", "Some", "None"];
      let response = Select::new("Select what to bump.", options).prompt()?;

      match response {
        "All" => {
          self.bump_all(packages, release)?;
          Ok(true)
        }
        "Some" => {
          let message = "Select the packages to bump.";
          let packages = MultiSelect::new(message, packages).prompt()?;
          self.bump_all(packages, release)?;
          Ok(true)
        }
        _ => Ok(false),
      }
    }
  }

  fn prompt_single(&self, package: Package, release: Release) -> Result<bool> {
    let message = format!("Bump {}?", package.name);
    let should_bump = Confirm::new(&message).with_default(true).prompt()?;

    if should_bump {
      self.bump(package, &release)?;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  fn bump(&self, package: Package, release: &Release) -> Result<()> {
    let mut builder = package.bump(release)?;
    
    if let Some(pre) = self.pre.as_deref() {
      builder.pre(pre)?;
    }

    if let Some(build) = &self.build {
      builder.build(build)?;
    }

    builder.bump()
  }

  fn bump_all(&self, packages: Vec<Package>, release: Release) -> Result<()> {
    for package in packages {
      self.bump(package, &release)?;
    }

    Ok(())
  }
}
