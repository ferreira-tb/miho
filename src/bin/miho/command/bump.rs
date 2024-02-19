use super::Choice;
use anyhow::{bail, Result};
use clap::Args;
use colored::Colorize;
use inquire::{Confirm, MultiSelect, Select, Text};
use miho::git::{Add, Commit, Git, Push};
use miho::package::Package;
use miho::release::Release;
use miho::search_packages;
use miho::version::VersionExt;

#[derive(Debug, Args)]
pub struct Bump {
  /// Type of the release.
  #[arg(default_value = "patch")]
  release: Option<String>,

  /// Include untracked files with `git add <PATHSPEC>`.
  #[arg(short = 'a', long, value_name = "PATHSPEC")]
  add: Option<String>,

  /// Build metadata.
  #[arg(long, value_name = "METADATA")]
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

  /// Package to bump.
  #[arg(short = 'P', long, value_name = "PACKAGE")]
  package: Option<Vec<String>>,

  /// Where to search for packages.
  #[arg(short = 'p', long, value_name = "PATH", default_value = ".")]
  path: Option<Vec<String>>,

  /// Prerelease identifier.
  #[arg(long, value_name = "IDENTIFIER")]
  pre: Option<String>,
}

impl super::Command for Bump {
  async fn execute(mut self) -> Result<()> {
    let path = self.path.as_deref().unwrap();
    let packages = search_packages!(path, self.package.as_deref())?;

    if packages.is_empty() {
      bail!("{}", "no valid package found".bold().red());
    }

    let release = self.release()?;

    preview(&packages, &release);

    if self.no_ask {
      bump_all(packages, &release)?;
    } else {
      let should_continue = prompt(packages, &release)?;
      if !should_continue {
        return Ok(());
      }
    }

    if !self.no_commit {
      self.commit().await?;
    }

    Ok(())
  }
}

impl Bump {
  async fn commit(&mut self) -> Result<()> {
    if let Some(pathspec) = &self.add {
      Add::new(pathspec).spawn().await?;
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
    commit.all();

    if self.no_verify {
      commit.no_verify();
    }

    commit.spawn().await?;

    if !self.no_push {
      Push::new().spawn().await?;
    }

    Ok(())
  }

  fn release(&self) -> Result<Release> {
    let mut parser = Release::parser();

    if let Some(pre) = self.pre.as_deref() {
      parser.prerelease(pre)?;
    }

    if let Some(build) = self.build.as_deref() {
      parser.metadata(build)?;
    }

    let release = self.release.as_deref().unwrap();
    let release = parser.parse(release)?;

    Ok(release)
  }
}

fn bump_all(packages: Vec<Package>, release: &Release) -> Result<()> {
  packages
    .into_iter()
    .try_for_each(|package| package.bump(release))
}

fn prompt(mut packages: Vec<Package>, release: &Release) -> Result<bool> {
  if packages.len() == 1 {
    let package = packages.swap_remove(0);
    prompt_single(package, release)
  } else {
    prompt_many(packages, release)
  }
}

fn prompt_single(package: Package, release: &Release) -> Result<bool> {
  let message = format!("Bump {}?", package.name);
  let should_bump = Confirm::new(&message).with_default(true).prompt()?;

  if should_bump {
    package.bump(release)?;
    Ok(true)
  } else {
    Ok(false)
  }
}

fn prompt_many(packages: Vec<Package>, release: &Release) -> Result<bool> {
  let options = vec![Choice::All, Choice::Some, Choice::None];
  let response = Select::new("Select what to bump.", options).prompt()?;

  match response {
    Choice::All => {
      bump_all(packages, release)?;
      Ok(true)
    }
    Choice::Some => {
      let message = "Select the packages to bump.";
      let packages = MultiSelect::new(message, packages).prompt()?;
      bump_all(packages, release)?;
      Ok(true)
    }
    Choice::None => Ok(false),
  }
}

fn preview(packages: &[Package], release: &Release) {
  use tabled::builder::Builder;
  use tabled::settings::object::Segment;
  use tabled::settings::{Alignment, Modify, Style};

  let mut builder = Builder::with_capacity(packages.len(), 5);

  for package in packages {
    let new_version = package.version.with_release(release);

    let agent = package
      .agent()
      .to_string()
      .to_uppercase()
      .bright_magenta()
      .bold();

    let record = [
      agent.to_string(),
      package.name.bold().to_string(),
      package.version.to_string().bright_blue().to_string(),
      "=>".to_string(),
      new_version.to_string().bright_green().to_string(),
    ];

    builder.push_record(record);
  }

  let mut table = builder.build();
  table.with(Style::blank());

  let version_col = Segment::new(.., 2..3);
  table.with(Modify::new(version_col).with(Alignment::right()));

  let new_version_col = Segment::new(.., 4..5);
  table.with(Modify::new(new_version_col).with(Alignment::right()));

  println!("{table}");
}
