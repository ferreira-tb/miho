use crate::util::search_packages;
use anyhow::{Context, Result};
use clap::Args;
use colored::Colorize;
use inquire::{Confirm, MultiSelect, Select, Text};
use miho::git::{Add, Commit, Git, Push};
use miho::package::builder::{self, Builder};
use miho::package::Package;
use miho::Release;
use semver::{BuildMetadata, Prerelease};
use std::fmt;

#[derive(Debug, Args)]
pub struct Bump {
  /// Type of the release.
  release: Option<String>,

  /// Include untracked files with `git add <PATHSPEC>`.
  #[arg(short = 'a', long, value_name = "PATHSPEC")]
  add: Option<String>,

  /// Build metadata.
  #[arg(short = 'B', long, value_name = "METADATA")]
  build: Option<String>,

  /// Commit the modified packages.
  #[arg(short = 'm', long, value_name = "MESSAGE")]
  commit_message: Option<String>,

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

  /// Where to search for packages.
  #[arg(short = 'p', long, value_name = "PATH")]
  path: Option<Vec<String>>,

  /// Prerelease identifier.
  #[arg(short = 'P', long, value_name = "IDENTIFIER")]
  pre: Option<String>,
}

impl Bump {
  pub async fn execute(&mut self) -> Result<()> {
    let path = self.path.as_deref().unwrap_or_default();
    let packages = search_packages(path)?;

    if packages.is_empty() {
      println!("{}", "No valid package found.".bold().red());
      return Ok(());
    }

    let release = match self.release.as_deref() {
      Some(r) => r.try_into()?,
      None => Release::Patch,
    };

    for package in &packages {
      self.preview(package, &release)?;
    }

    if self.no_ask {
      self.bump_all(packages, release)?;
    } else {
      let should_continue = self.prompt(packages, release)?;
      if !should_continue {
        return Ok(());
      }
    }

    if !self.no_commit {
      self.commit().await?;
    }

    Ok(())
  }

  fn prompt(&self, mut packages: Vec<Package>, release: Release) -> Result<bool> {
    if packages.len() == 1 {
      let package = packages.swap_remove(0);
      self.prompt_single(package, release)
    } else {
      self.prompt_many(packages, release)
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

  fn prompt_many(&self, packages: Vec<Package>, release: Release) -> Result<bool> {
    let options = vec![Prompt::All, Prompt::Some, Prompt::None];
    let response = Select::new("Select what to bump.", options).prompt()?;

    match response {
      Prompt::All => {
        self.bump_all(packages, release)?;
        Ok(true)
      }
      Prompt::Some => {
        let message = "Select the packages to bump.";
        let packages = MultiSelect::new(message, packages).prompt()?;
        self.bump_all(packages, release)?;
        Ok(true)
      }
      Prompt::None => Ok(false),
    }
  }

  fn bump(&self, package: Package, release: &Release) -> Result<()> {
    let mut bump = builder::Bump::new(package, release);

    if let Some(pre) = self.pre.as_deref() {
      bump.pre(pre)?;
    }

    if let Some(build) = self.build.as_deref() {
      bump.build(build)?;
    }

    bump.execute()?;

    Ok(())
  }

  fn bump_all(&self, packages: Vec<Package>, release: Release) -> Result<()> {
    packages
      .into_iter()
      .try_for_each(|package| self.bump(package, &release))
  }

  async fn commit(&mut self) -> Result<()> {
    if let Some(pathspec) = &self.add {
      Add::new(pathspec)
        .spawn()
        .await
        .with_context(|| "failed to update git index")?;
    }

    let message = if !self.no_ask && self.commit_message.is_none() {
      Text::new("Commit message: ").prompt_skippable()?
    } else {
      self.commit_message.take()
    };

    let message = match message.as_deref().map(str::trim) {
      Some(m) if !m.is_empty() => m,
      _ => "chore: bump version",
    };

    let mut commit = Commit::new(message);

    if self.no_verify {
      commit.no_verify();
    }

    commit
      .all()
      .spawn()
      .await
      .with_context(|| "failed to commit packages")?;

    if !self.no_push {
      Push::new()
        .spawn()
        .await
        .with_context(|| "failed to push commit")?;
    }

    Ok(())
  }

  fn preview(&self, package: &Package, release: &Release) -> Result<()> {
    let pre = self.pre.as_deref();

    let mut new_version = match pre {
      Some(pre) => release.increment_pre(&package.version, Prerelease::new(pre)?),
      None => release.increment(&package.version),
    };

    if let Some(build) = self.build.as_deref() {
      new_version.build = BuildMetadata::new(build)?;
    }

    println!(
      "[ {}: {} ]  {}  =>  {}",
      package
        .agent()
        .to_string()
        .to_uppercase()
        .bright_magenta()
        .bold(),
      package.name.bold(),
      package.version.to_string().bright_blue(),
      new_version.to_string().bright_green()
    );

    Ok(())
  }
}

enum Prompt {
  All,
  Some,
  None,
}

impl fmt::Display for Prompt {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::All => write!(f, "All"),
      Self::Some => write!(f, "Some"),
      Self::None => write!(f, "None"),
    }
  }
}
