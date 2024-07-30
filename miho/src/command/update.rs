use super::{Choice, Commit, PromptResult};
use crate::agent::Agent;
use crate::command;
use crate::dependency::{Dependency, DependencyTree};
use crate::package::{GlobalPackage, Package, PackageDependencyTree, PackageDisplay};
use crate::prelude::*;
use crate::release::Release;
use crate::version::ComparatorExt;
use ahash::{HashSet, HashSetExt};
use clap::Args;
use crossterm::{cursor, terminal, ExecutableCommand};
use future_iter::join_set::IntoJoinSetBy;
use inquire::{MultiSelect, Select};
use semver::Comparator;
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::{env, fmt, mem};
use strum::IntoEnumIterator;
use tokio::process::Command;
use tokio::task::JoinSet;

type TreeTuple<T> = (T, DependencyTree);

static RELEASE: OnceLock<Option<Release>> = OnceLock::new();

#[derive(Debug, Args, miho_derive::Commit)]
pub struct Update {
  /// Type of the release.
  release: Option<String>,

  /// Include untracked files with `git add <PATHSPEC>`.
  #[arg(short = 'a', long, value_name = "PATHSPEC")]
  add: Option<String>,

  /// Commit the modified packages.
  #[arg(short = 'm', long, value_name = "MESSAGE")]
  commit_message: Option<String>,

  /// Dependencies to update.
  #[arg(short = 'D', long, value_name = "DEPENDENCY")]
  dependency: Option<Vec<String>>,

  /// Show preview and exit without updating.
  #[arg(short = 'd', long)]
  dry_run: bool,

  /// Update global dependencies.
  #[arg(short = 'g', long)]
  global: bool,

  /// Do not ask for consent before updating.
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

  /// Package to update.
  #[arg(short = 'P', long, value_name = "PACKAGE")]
  package: Option<Vec<String>>,

  /// Where to search for packages.
  #[arg(short = 'p', long, value_name = "PATH", default_value = ".")]
  path: Option<Vec<PathBuf>>,

  /// Whether to only update peer dependencies.
  #[arg(long)]
  peer: bool,
}

impl super::Command for Update {
  async fn execute(mut self) -> Result<()> {
    trace!(command = ?self);
    self.set_release();

    if self.global {
      self.execute_global().await
    } else {
      self.execute_local().await
    }
  }
}

impl Update {
  fn set_release(&self) {
    let release = self.release.as_deref().and_then(|it| {
      let release = Release::parser().parse(it).ok();
      release.filter(Release::is_stable)
    });

    RELEASE.set(release).unwrap();
  }

  async fn execute_local(&mut self) -> Result<()> {
    let path = self
      .path
      .as_deref()
      .expect("should have `.` as the default value");

    let only = self.package.as_deref();
    let packages = Package::search(path, only)?;
    let mut trees = self.fetch(packages).await?;

    if trees.is_empty() {
      println!("{}", "all dependencies are up to date".bright_green());
      return Ok(());
    }

    preview(&trees);

    if self.dry_run {
      return Ok(());
    }

    if self.no_ask {
      update_local(trees).await?;
    } else {
      match prompt(&mut trees)? {
        PromptResult::Abort => return Ok(()),
        PromptResult::Continue => {
          update_local(trees).await?;
        }
      }
    }

    if !self.no_commit {
      self.commit("chore: bump dependencies").await?;
    }

    Ok(())
  }

  async fn execute_global(&self) -> Result<()> {
    let packages = GlobalPackage::get().await?;
    let mut trees = self.fetch(packages).await?;

    if trees.is_empty() {
      println!("{}", "all dependencies are up to date".bright_green());
      return Ok(());
    }

    preview(&trees);

    if self.dry_run {
      return Ok(());
    }

    if self.no_ask {
      update_global(trees).await
    } else {
      match prompt(&mut trees)? {
        PromptResult::Abort => Ok(()),
        PromptResult::Continue => update_global(trees).await,
      }
    }
  }

  async fn fetch<T>(&self, packages: Vec<T>) -> Result<Vec<TreeTuple<T>>>
  where
    T: PackageDependencyTree + PackageDisplay + Ord + Send + Sync + 'static,
  {
    let total_amount = packages.len();
    let trees: Vec<TreeTuple<T>> = Vec::with_capacity(total_amount);
    let trees = Arc::new(Mutex::new(trees));

    update_fetch_progress(0, total_amount)?;

    let cache = Arc::new(Mutex::new(HashSet::new()));
    let mut set: JoinSet<Result<()>> = packages.into_join_set_by(|package| {
      let trees = Arc::clone(&trees);
      let cache = Arc::clone(&cache);
      async move {
        let mut tree = package.dependency_tree();
        tree.fetch(cache).await?;

        let mut trees = trees.lock().unwrap();
        trees.push((package, tree));

        clear_line()?;
        update_fetch_progress(trees.len(), total_amount)?;

        Ok(())
      }
    });

    while let Some(result) = set.join_next().await {
      result??;
    }

    clear_line()?;

    let trees = Arc::into_inner(trees)
      .unwrap()
      .into_inner()?
      .into_iter()
      .filter_map(|(package, mut tree)| {
        self.filter_dependencies(&mut tree);

        if tree.dependencies.is_empty() {
          None
        } else {
          tree.dependencies.sort_unstable();
          Some((package, tree))
        }
      });

    let mut trees = trees.collect_vec();
    trees.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    Ok(trees)
  }

  fn filter_dependencies(&self, tree: &mut DependencyTree) {
    let release = RELEASE.get().unwrap();
    let chosen_deps = self.dependency.as_deref().unwrap_or_default();

    tree.dependencies.retain(|dependency| {
      if !chosen_deps.is_empty() && !chosen_deps.contains(&dependency.name) {
        return false;
      }

      if self.peer && !dependency.kind.is_peer() {
        return false;
      }

      if !self.peer && dependency.kind.is_peer() {
        return false;
      }

      dependency.target_cmp(release).is_some()
    });
  }
}

async fn update_local(trees: Vec<TreeTuple<Package>>) -> Result<()> {
  let release = RELEASE.get().unwrap();
  let agents = trees
    .iter()
    .map(|(package, _)| package.agent())
    .unique()
    .collect_vec();

  for (package, tree) in trees {
    package.update(tree, release)?;
  }

  if let Some(agent) = agents.iter().find(|it| it.is_node()) {
    let cwd = env::current_dir()?;
    let lockfile = agent.lockfile().unwrap();
    let lockfile = cwd.join(lockfile);

    if let Ok(true) = lockfile.try_exists() {
      let program = agent.to_string().to_lowercase();
      command!(&program)
        .arg("install")
        .spawn()?
        .wait()
        .await?;
    }
  }

  if agents.contains(&Agent::Cargo) {
    Command::new("cargo")
      .arg("update")
      .spawn()?
      .wait()
      .await?;
  }

  Ok(())
}

async fn update_global(trees: Vec<TreeTuple<GlobalPackage>>) -> Result<()> {
  let release = RELEASE.get().unwrap();
  for (package, tree) in trees {
    package.update(tree, release).await?;
  }

  Ok(())
}

fn prompt(trees: &mut Vec<(impl PackageDisplay, DependencyTree)>) -> Result<PromptResult> {
  let options = Choice::iter().collect();
  let choice = Select::new("Update dependencies?", options).prompt()?;

  match choice {
    Choice::All => Ok(PromptResult::Continue),
    Choice::None => Ok(PromptResult::Abort),
    Choice::Some => {
      struct Wrapper(Dependency);

      impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{}", self.0.name)
        }
      }

      for (package, tree) in trees.iter_mut() {
        let message = package.display();
        let dependencies = mem::take(&mut tree.dependencies);
        let dependencies = dependencies.into_iter().map(Wrapper).collect();

        let dependencies = MultiSelect::new(&message, dependencies)
          .with_all_selected_by_default()
          .prompt()?;

        tree.dependencies = dependencies.into_iter().map(|d| d.0).collect();
      }

      trees.retain(|(_, tree)| !tree.dependencies.is_empty());

      if trees.is_empty() {
        println!("{}", "no dependencies selected".truecolor(105, 105, 105));
        Ok(PromptResult::Abort)
      } else {
        Ok(PromptResult::Continue)
      }
    }
  }
}

fn preview(trees: &[(impl PackageDisplay, DependencyTree)]) {
  use tabled::builder::Builder;
  use tabled::settings::object::Segment;
  use tabled::settings::{Alignment, Modify, Panel, Style};

  let release = RELEASE.get().unwrap();
  let mut tables = Vec::with_capacity(trees.len());

  for (package, tree) in trees {
    let dep_amount = tree.dependencies.len();
    let mut builder = Builder::with_capacity(dep_amount, 6);

    for dependency in &tree.dependencies {
      if let Some(target_cmp) = dependency.target_cmp(release) {
        let comparator = &dependency.comparator;

        let mut record = vec![
          dependency.name.clone(),
          dependency.kind.as_ref().bright_cyan().to_string(),
          comparator.to_string().bright_blue().to_string(),
          "=>".to_string(),
          target_cmp.to_string().bright_green().to_string(),
        ];

        if let Some(latest) = dependency.latest() {
          let latest_cmp = Comparator::from_version(latest, comparator.op);
          if latest_cmp.pre.is_empty() && latest_cmp != target_cmp {
            let latest = format!("({latest} available)");
            record.push(latest.truecolor(105, 105, 105).to_string());
          }
        }

        builder.push_record(record);
      }
    }

    if builder.count_records() == 0 {
      continue;
    }

    let mut table = builder.build();
    let header = package.display();
    table
      .with(Style::blank())
      .with(Panel::header(header));

    let version_col = Segment::new(.., 2..3);
    table.with(Modify::new(version_col).with(Alignment::right()));

    let new_version_col = Segment::new(.., 4..5);
    table.with(Modify::new(new_version_col).with(Alignment::right()));

    tables.push(table);
  }

  let mut tables = tables.into_iter().peekable();
  while let Some(table) = tables.next() {
    let mut table = format!("{table}");

    if tables.peek().is_some() {
      table.push('\n');
    }

    println!("{table}");
  }
}

fn update_fetch_progress(current: usize, total: usize) -> Result<()> {
  let progress = format!("({current}/{total})");
  let mut stdout = io::stdout().lock();

  writeln!(
    stdout,
    "{} {}",
    "fetching dependencies...".bright_cyan(),
    progress.truecolor(105, 105, 105)
  )?;

  stdout.flush()?;

  Ok(())
}

fn clear_line() -> Result<()> {
  let mut stdout = io::stdout().lock();
  stdout.execute(cursor::MoveUp(1))?;
  stdout.execute(terminal::Clear(terminal::ClearType::FromCursorDown))?;
  stdout.flush()?;

  Ok(())
}
