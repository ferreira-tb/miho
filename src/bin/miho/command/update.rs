use anyhow::{bail, Result};
use clap::Args;
use colored::Colorize;
use miho::package::dependency::Tree;
use miho::package::Package;
use miho::release::Release;
use miho::version::{Comparator, ComparatorExt, VersionExt};
use std::sync::{Arc, Mutex};
use tokio::task::JoinSet;

#[derive(Debug, Args)]
pub struct Update {
  /// Type of the release.
  release: Option<String>,

  /// Install the updated packages.
  #[arg(short = 'i', long)]
  install: bool,

  /// Do not ask for consent before updating.
  #[arg(short = 'k', long)]
  no_ask: bool,

  /// Where to search for packages.
  #[arg(short = 'p', long, value_name = "PATH", default_value = ".")]
  path: Option<Vec<String>>,
}

impl super::Command for Update {
  async fn execute(self) -> Result<()> {
    let path = self.path.as_deref().unwrap();
    let packages = Package::search(path)?;

    if packages.is_empty() {
      bail!("{}", "No valid package found.".bold().red());
    }

    let release = self.release();
    let trees = fetch_trees(packages).await?;

    if trees.is_empty() {
      todo!("ADD ERROR MESSAGE");
    }

    preview(&trees, &release);

    if self.no_ask {
      update_all(trees, &release)?;
    } else {
      prompt(trees, &release);
    }

    Ok(())
  }
}

impl Update {
  fn release(&self) -> Option<Release> {
    if let Some(release) = self.release.as_deref() {
      let release = Release::parser().parse(release).ok();
      release.filter(Release::is_stable)
    } else {
      None
    }
  }
}

async fn fetch_trees(packages: Vec<Package>) -> Result<Vec<(Package, Tree)>> {
  let mut set: JoinSet<Result<()>> = JoinSet::new();
  let trees = Vec::with_capacity(packages.len());
  let trees = Arc::new(Mutex::new(trees));

  for package in packages {
    let trees = Arc::clone(&trees);
    set.spawn(async move {
      let mut tree = package.dependency_tree();
      tree.fetch_metadata().await?;

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

fn update_all(trees: Vec<(Package, Tree)>, release: &Option<Release>) -> Result<()> {
  trees
    .into_iter()
    .try_for_each(|(package, tree)| package.update(tree, release))
}

fn prompt(_trees: Vec<(Package, Tree)>, _release: &Option<Release>) {
  unimplemented!()
}

fn preview(trees: &[(Package, Tree)], release: &Option<Release>) {
  use tabled::builder::Builder;
  use tabled::settings::object::Segment;
  use tabled::settings::{Alignment, Modify, Panel, Style};

  let mut tables = Vec::with_capacity(trees.len());

  for (package, tree) in trees {
    let dep_amount = tree.dependencies.len();
    let mut builder = Builder::with_capacity(dep_amount, 6);

    for dependency in &tree.dependencies {
      let comparator = &dependency.comparator;
      let requirement = if let Some(r) = release {
        comparator.with_release(r).as_version_req()
      } else {
        comparator.as_version_req()
      };

      if let Some(target) = dependency.latest_with_req(&requirement) {
        let target_cmp = target.as_comparator(comparator.op);
        if target_cmp == *comparator {
          continue;
        }

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

    let header = format!(
      "[ {} ] {}",
      package
        .agent()
        .to_string()
        .to_uppercase()
        .bright_magenta()
        .bold(),
      package.name.bright_yellow().bold()
    );

    let mut table = builder.build();
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
