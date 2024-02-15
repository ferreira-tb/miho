use crate::util::search_packages;
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use miho::package::dependency::Tree;
use miho::package::Package;
use miho::version::{Comparator, ComparatorExt, VersionReq, VersionReqExt};
use miho::Release;
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
  #[arg(short = 'p', long, value_name = "PATH")]
  path: Option<Vec<String>>,
}

impl super::Command for Update {
  async fn execute(self) -> Result<()> {
    let path = self.path.as_deref().unwrap_or_default();
    let packages = search_packages(path)?;

    if packages.is_empty() {
      println!("{}", "No valid package found.".bold().red());
      return Ok(());
    }

    let release = self.release();
    let trees = self.trees(packages).await?;

    if trees.is_empty() {
      todo!("ADD ERROR MESSAGE");
    }

    Self::preview(&trees, release.as_ref());

    if self.no_ask {
    } else {
      todo!("ADD PROMPT");
    }

    Ok(())
  }
}

impl Update {
  async fn fetch_trees(&self, packages: Vec<Package>) -> Result<Vec<(Package, Tree)>> {
    let mut set: JoinSet<Result<()>> = JoinSet::new();
    let trees = Vec::with_capacity(packages.len());
    let trees = Arc::new(Mutex::new(trees));

    for package in packages {
      let tree_vec = Arc::clone(&trees);
      set.spawn(async move {
        let mut tree = package.dependency_tree();
        tree.fetch_metadata().await?;

        let mut tree_vec = tree_vec.lock().unwrap();
        tree_vec.push((package, tree));

        Ok(())
      });
    }

    while let Some(result) = set.join_next().await {
      result??;
    }

    let mutex = Arc::into_inner(trees).unwrap();
    let trees = mutex.into_inner()?;

    Ok(trees)
  }

  fn release(&self) -> Option<Release> {
    if let Some(release) = self.release.as_deref() {
      let release = Release::parser().parse(release).ok();
      release.filter(Release::is_stable)
    } else {
      None
    }
  }

  async fn trees(&self, packages: Vec<Package>) -> Result<Vec<(Package, Tree)>> {
    let trees = self
      .fetch_trees(packages)
      .await?
      .into_iter()
      .filter_map(|(package, mut tree)| {
        if tree.dependencies.is_empty() {
          None
        } else {
          tree.dependencies.sort_unstable();
          Some((package, tree))
        }
      });

    Ok(trees.collect())
  }

  fn preview(trees: &[(Package, Tree)], release: Option<&Release>) {
    use tabled::builder::Builder;
    use tabled::settings::object::Segment;
    use tabled::settings::{Alignment, Modify, Panel, Style};

    let mut tables = Vec::with_capacity(trees.len());

    for (package, tree) in trees {
      let dep_amount = tree.dependencies.len();
      let mut builder = Builder::with_capacity(dep_amount, 2);

      for dependency in &tree.dependencies {
        let comparator = &dependency.comparator;
        let mut requirement = VersionReq::from_comparator(comparator);

        if let Some(release) = release {
          requirement
            .comparators
            .push(comparator.with_release(release));
        }

        if let Some(max) = dependency.max(&requirement) {
          let op = comparator.op;
          let max = Comparator::from_version(max, op);

          if max == *comparator {
            continue;
          }

          builder.push_record([
            dependency.name.clone(),
            dependency.kind.to_string().bright_cyan().to_string(),
            comparator.to_string().bright_blue().to_string(),
            "=>".to_string(),
            max.to_string().bright_green().to_string(),
          ]);
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
}
