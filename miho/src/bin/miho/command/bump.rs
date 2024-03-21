use super::{Choice, Commit};
use anyhow::{bail, Result};
use clap::Args;
use colored::Colorize;
use inquire::{Confirm, MultiSelect, Select};
use miho::package::Package;
use miho::release::Release;
use miho::search_packages;
use miho::version::VersionExt;
use std::fmt;
use std::path::PathBuf;
use std::sync::OnceLock;

static RELEASE: OnceLock<Release> = OnceLock::new();

#[derive(Debug, Args, miho_derive::Commit)]
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

impl super::Command for Bump {
  async fn execute(mut self) -> Result<()> {
    let path = self.path.as_deref().unwrap();
    let packages = search_packages!(path, self.package.as_deref())?;

    if packages.is_empty() {
      bail!("{}", "no valid package found".bold().red());
    }

    self.set_release()?;
    preview(&packages);

    if self.no_ask {
      bump_all(packages)?;
    } else {
      let should_continue = prompt(packages)?;
      if !should_continue {
        return Ok(());
      }
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

    let release = self.release.as_deref().unwrap();
    let release = parser.parse(release)?;
    RELEASE.set(release).unwrap();

    Ok(())
  }
}

fn bump_all(packages: Vec<Package>) -> Result<()> {
  let release = RELEASE.get().unwrap();
  packages
    .into_iter()
    .try_for_each(|package| package.bump(release))
}

fn prompt(mut packages: Vec<Package>) -> Result<bool> {
  if packages.len() == 1 {
    let package = packages.swap_remove(0);
    prompt_single(package)
  } else {
    prompt_many(packages)
  }
}

fn prompt_single(package: Package) -> Result<bool> {
  let message = format!("Bump {}?", package.name);
  let should_bump = Confirm::new(&message).with_default(true).prompt()?;

  if should_bump {
    let release = RELEASE.get().unwrap();
    package.bump(release)?;
    Ok(true)
  } else {
    Ok(false)
  }
}

fn prompt_many(packages: Vec<Package>) -> Result<bool> {
  let options = vec![Choice::All, Choice::Some, Choice::None];
  let choice = Select::new("Bump packages?", options).prompt()?;

  match choice {
    Choice::All => {
      bump_all(packages)?;
      Ok(true)
    }
    Choice::Some => {
      struct Wrapper(Package);

      impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          let agent = self.0.agent().to_string().to_uppercase();
          write!(f, "{agent}: {}", self.0.name)
        }
      }

      let message = "Select the packages to bump.";
      let packages: Vec<Wrapper> = packages.into_iter().map(Wrapper).collect();
      let packages = MultiSelect::new(message, packages).prompt()?;

      if packages.is_empty() {
        println!("{}", "no package selected".truecolor(105, 105, 105));
        Ok(false)
      } else {
        bump_all(packages.into_iter().map(|p| p.0).collect())?;
        Ok(true)
      }
    }
    Choice::None => Ok(false),
  }
}

fn preview(packages: &[Package]) {
  use tabled::builder::Builder;
  use tabled::settings::object::Segment;
  use tabled::settings::{Alignment, Modify, Style};

  let release = RELEASE.get().unwrap();
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
