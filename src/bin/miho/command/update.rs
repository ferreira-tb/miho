use super::Choice;
use anyhow::{bail, Result};
use clap::Args;
use colored::Colorize;
use inquire::{MultiSelect, Select};
use miho::package::dependency::{self, Dependency, Tree};
use miho::package::Package;
use miho::release::Release;
use miho::search_packages;
use miho::version::{Comparator, ComparatorExt};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use std::{fmt, mem};
use tokio::task::JoinSet;

static RELEASE: OnceLock<Option<Release>> = OnceLock::new();

#[derive(Debug, Args)]
pub struct Update {
  /// Type of the release.
  release: Option<String>,

  /// Do not ask for consent before updating.
  #[arg(short = 'k', long)]
  no_ask: bool,

  /// Package to update.
  #[arg(short = 'P', long, value_name = "PACKAGE")]
  package: Option<Vec<String>>,

  /// Where to search for packages.
  #[arg(short = 'p', long, value_name = "PATH", default_value = ".")]
  path: Option<Vec<PathBuf>>,

  /// Whether to include peer dependencies.
  #[arg(long)]
  peer: bool,
}

impl super::Command for Update {
  async fn execute(self) -> Result<()> {
    let path = self.path.as_deref().unwrap();
    let packages = search_packages!(path, self.package.as_deref())?;

    if packages.is_empty() {
      bail!("{}", "no valid package found".bold().red());
    }

    self.set_release();
    let trees = self.fetch(packages).await?;

    if trees.is_empty() {
      println!("{}", "all dependencies are up to date".bright_green());
      return Ok(());
    }

    preview(&trees);

    if self.no_ask {
      update_all(trees)?;
    } else {
      prompt(trees)?;
    }

    Ok(())
  }
}

impl Update {
  fn set_release(&self) {
    let release = self.release.as_deref().and_then(|release| {
      let release = Release::parser().parse(release).ok();
      release.filter(Release::is_stable)
    });

    RELEASE.set(release).unwrap();
  }

  async fn fetch(&self, packages: Vec<Package>) -> Result<Vec<(Package, Tree)>> {
    let mut set: JoinSet<Result<()>> = JoinSet::new();
    let trees = Vec::with_capacity(packages.len());
    let trees = Arc::new(Mutex::new(trees));

    for package in packages {
      let trees = Arc::clone(&trees);
      set.spawn(async move {
        let mut tree = package.dependency_tree();
        tree.fetch().await?;

        let mut trees = trees.lock().unwrap();
        trees.push((package, tree));

        Ok(())
      });
    }

    while let Some(result) = set.join_next().await {
      result??;
    }

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

    let mut trees: Vec<(Package, Tree)> = trees.collect();
    trees.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));

    Ok(trees)
  }

  fn filter_dependencies(&self, tree: &mut Tree) {
    tree.dependencies.retain(|dependency| {
      if dependency.kind == dependency::Kind::Peer && !self.peer {
        return false;
      }

      let release = RELEASE.get().unwrap();
      dependency.target_cmp(release).is_some()
    });
  }
}

fn update_all(trees: Vec<(Package, Tree)>) -> Result<()> {
  let release = RELEASE.get().unwrap();
  trees
    .into_iter()
    .try_for_each(|(package, tree)| package.update(tree, release))
}

fn prompt(mut trees: Vec<(Package, Tree)>) -> Result<()> {
  let options = vec![Choice::All, Choice::Some, Choice::None];
  let response = Select::new("Update dependencies?", options).prompt()?;

  match response {
    Choice::All => {
      update_all(trees)?;
      Ok(())
    }
    Choice::Some => {
      struct Wrapper(Dependency);

      impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
          write!(f, "{}", self.0.name)
        }
      }

      for (package, tree) in &mut trees {
        let message = display_package(package);
        let dependencies = mem::take(&mut tree.dependencies);
        let dependencies: Vec<Wrapper> = dependencies.into_iter().map(Wrapper).collect();

        let length = dependencies.len();
        let dependencies = MultiSelect::new(&message, dependencies)
          .with_default(&((0..length).collect::<Vec<_>>()))
          .prompt()?;

        tree.dependencies = dependencies.into_iter().map(|d| d.0).collect();
      }

      trees.retain(|(_, tree)| !tree.dependencies.is_empty());

      if trees.is_empty() {
        println!("{}", "no dependencies selected".truecolor(105, 105, 105));
        Ok(())
      } else {
        update_all(trees)?;
        Ok(())
      }
    }
    Choice::None => Ok(()),
  }
}

fn preview(trees: &[(Package, Tree)]) {
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
          dependency.kind.to_string().bright_cyan().to_string(),
          comparator.to_string().bright_blue().to_string(),
          "=>".to_string(),
          target_cmp.to_string().bright_green().to_string(),
        ];

        if let Some(latest) = dependency.latest() {
          let latest_cmp = Comparator::from_version(latest, comparator.op);
          if latest_cmp != target_cmp {
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
    let header = display_package(package);
    table.with(Style::blank()).with(Panel::header(header));

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

fn display_package(package: &Package) -> String {
  format!(
    "[ {} ] {}",
    package
      .agent()
      .to_string()
      .to_uppercase()
      .bright_magenta()
      .bold(),
    package.name.bright_yellow().bold()
  )
}
