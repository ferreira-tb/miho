use anyhow::Result;
use clap::Args;
use colored::Colorize;
use inquire::{Confirm, MultiSelect, Select};
use itertools::Itertools;
use serde::Deserialize;
use std::fmt;
use std::path::PathBuf;
use std::sync::OnceLock;
use strum::IntoEnumIterator;
use tokio::process::Command;

use super::{Choice, Commit, PromptResult};
use crate::agent::Agent;
use crate::package::Package;
use crate::package::manifest::DEFAULT_VERSION;
use crate::release::Release;
use crate::version::VersionExt;
use crate::{impl_commit, search_packages};

static RELEASE: OnceLock<Release> = OnceLock::new();

#[derive(Args, Debug, Default, Deserialize)]
#[serde(default)]
pub struct Bump {
  /// Type of the release.
  #[arg(default_value = "patch")]
  release: Option<String>,

  /// Include untracked files with `git add <PATHSPEC>`.
  #[arg(short = 'a', long, value_name = "PATHSPEC")]
  add: Option<String>,

  /// Only bump packages with the specified agent.
  #[arg(short = 'A', long, value_name = "AGENT")]
  agent: Option<Vec<String>>,

  /// Build metadata.
  #[arg(long, value_name = "METADATA")]
  build: Option<String>,

  /// Show preview and exit without bumping.
  #[arg(short = 'd', long)]
  dry_run: bool,

  /// Commit the modified packages.
  #[arg(short = 'm', long, value_name = "MESSAGE")]
  commit_message: Option<String>,

  /// Do not ask for consent before bumping.
  #[arg(short = 'k', long)]
  no_ask: bool,

  /// Do not commit the modified packages.
  #[arg(short = 't', long)]
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
  path: Option<Vec<PathBuf>>,

  /// Prerelease identifier.
  #[arg(long, value_name = "IDENTIFIER")]
  pre: Option<String>,
}

impl_commit!(Bump);

impl super::Command for Bump {
  async fn execute(mut self) -> Result<()> {
    self.set_release()?;

    let packages = search_packages!(&self)
      .into_iter()
      .filter(|it| it.version != DEFAULT_VERSION)
      .collect_vec();

    preview(&packages);

    if self.dry_run {
      return Ok(());
    }

    if self.no_ask {
      bump_all(packages).await?;
    } else if let PromptResult::Abort = prompt(packages).await? {
      return Ok(());
    }

    if !self.no_commit {
      self.commit("chore: bump version").await?;
    }

    Ok(())
  }
}

impl Bump {
  fn set_release(&self) -> Result<()> {
    let mut parser = Release::parser();

    if let Some(pre) = self.pre.as_deref() {
      parser.prerelease(pre)?;
    }

    if let Some(build) = self.build.as_deref() {
      parser.metadata(build)?;
    }

    let release = self
      .release
      .as_deref()
      .expect("should have `patch` as the default value");

    let release = parser.parse(release)?;

    RELEASE.set(release).unwrap();

    Ok(())
  }
}

async fn bump_all(packages: Vec<Package>) -> Result<()> {
  let release = RELEASE.get().unwrap();
  let agents = packages
    .iter()
    .map(Package::agent)
    .unique()
    .collect_vec();

  packages
    .into_iter()
    .try_for_each(|package| package.bump(release))?;

  // https://doc.rust-lang.org/cargo/commands/cargo-update.html#update-options
  if agents.contains(&Agent::Cargo) {
    Command::new("cargo")
      .args(["update", "--workspace"])
      .spawn()?
      .wait()
      .await?;
  }

  Ok(())
}

async fn prompt(mut packages: Vec<Package>) -> Result<PromptResult> {
  if packages.len() == 1 {
    let package = packages.swap_remove(0);
    prompt_one(package)
  } else {
    prompt_many(packages).await
  }
}

fn prompt_one(package: Package) -> Result<PromptResult> {
  let message = format!("Bump {}?", package.name);
  let should_bump = Confirm::new(&message)
    .with_default(true)
    .prompt()?;

  if should_bump {
    let release = RELEASE.get().unwrap();
    package.bump(release)?;
    Ok(PromptResult::Continue)
  } else {
    Ok(PromptResult::Abort)
  }
}

async fn prompt_many(packages: Vec<Package>) -> Result<PromptResult> {
  let options = Choice::iter().collect_vec();
  let choice = Select::new("Bump packages?", options).prompt()?;

  match choice {
    Choice::All => {
      bump_all(packages).await?;
      Ok(PromptResult::Continue)
    }
    Choice::Some => {
      let message = "Select the packages to bump.";
      let packages = packages.into_iter().map(ChoiceWrapper).collect();
      let packages = MultiSelect::new(message, packages).prompt()?;

      if packages.is_empty() {
        println!("{}", "no package selected".truecolor(105, 105, 105));
        Ok(PromptResult::Abort)
      } else {
        let packages = packages.into_iter().map(|it| it.0).collect();
        bump_all(packages).await?;
        Ok(PromptResult::Continue)
      }
    }
    Choice::None => Ok(PromptResult::Abort),
  }
}

fn preview(packages: &[Package]) {
  use tabled::builder::Builder;
  use tabled::settings::object::Segment;
  use tabled::settings::{Alignment, Modify, Style};

  let release = RELEASE.get().unwrap();
  let mut builder = Builder::with_capacity(packages.len(), 5);

  for package in packages {
    let agent = package
      .agent()
      .to_string()
      .bright_magenta()
      .bold();

    let version = package
      .version
      .to_string()
      .bright_blue()
      .to_string();

    let new_version = package
      .version
      .with_release(release)
      .to_string()
      .bright_green()
      .to_string();

    let record = [
      agent.to_string(),
      package.name.bold().to_string(),
      version,
      "=>".to_string(),
      new_version,
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

struct ChoiceWrapper(Package);

impl fmt::Display for ChoiceWrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let agent = self.0.agent().to_string();
    write!(f, "{agent}: {}", self.0.name)
  }
}
