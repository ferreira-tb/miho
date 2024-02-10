use anyhow::{Context, Result};
use clap::Args;
use colored::*;
use inquire::{Confirm, MultiSelect, Select, Text};
use miho::git::{Add, Commit, GitCommand, Push};
use miho::{BumpBuilder, Package, Release, SearchBuilder};
use semver::Prerelease;
use std::process::Stdio;

#[derive(Debug, Args)]
pub struct Bump {
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

impl super::Command for Bump {
  fn execute(&mut self) -> Result<()> {
    let packages = match &self.globs {
      Some(globs) if !globs.is_empty() => {
        let mut globs: Vec<&str> = globs.iter().map(|g| g.as_str()).collect();
        let last = globs.pop().expect("globs is not empty");
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
      Some(r) => r.try_into()?,
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
      let should_continue = self.prompt_bump(packages, release)?;
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

      if !self.no_ask {
        self.prompt_commit_message()?;
      }

      let message = self.commit_message.as_deref().map(|m| m.trim());
      let message = match message {
        Some(m) if !m.is_empty() => m,
        _ => "chore: bump version",
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
}

impl Bump {
  fn prompt_bump(&self, mut packages: Vec<Package>, release: Release) -> Result<bool> {
    if packages.len() == 1 {
      let package = packages.swap_remove(0);
      let message = format!("Bump {}?", package.name);
      let should_bump = Confirm::new(&message).with_default(true).prompt()?;

      if should_bump {
        self.bump(package, &release)?;
        Ok(true)
      } else {
        Ok(false)
      }
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

  fn prompt_commit_message(&mut self) -> Result<()> {
    let message = Text::new("Commit message: ").prompt_skippable()?;
    if let Some(message) = message {
      self.commit_message = Some(message);
    }

    Ok(())
  }

  fn bump(&self, package: Package, release: &Release) -> Result<()> {
    let mut builder = BumpBuilder::new(&package, release);

    if let Some(pre) = self.pre.as_deref() {
      builder.pre(pre)?;
    }

    if let Some(build) = self.build.as_deref() {
      builder.build(build)?;
    }

    builder.bump()?;

    Ok(())
  }

  fn bump_all(&self, packages: Vec<Package>, release: Release) -> Result<()> {
    for package in packages {
      self.bump(package, &release)?;
    }

    Ok(())
  }
}
