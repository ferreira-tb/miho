use crate::util::search_packages;
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use miho::package::dependency::Tree;
use miho::package::{Agent, Package};
use miho::Release;
use std::convert::TryInto;
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

    let _release = self.release();
    let trees = self.fetch_trees(packages).await?;
    let trees: Vec<(Package, Tree)> = trees
      .into_iter()
      .filter(|(_, tree)| tree.agent != Agent::Cargo)
      .collect();

    for (package, tree) in &trees {
      if !tree.dependencies.is_empty() {
        Self::preview(package, tree);
      }
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
    self
      .release
      .as_deref()
      .map(TryInto::try_into)
      .transpose()
      .unwrap_or(None)
  }

  fn preview(package: &Package, tree: &Tree) {
    let iter = tree.dependencies.iter().filter_map(|dep| {
      if let Some(max) = dep.max() {
        let line = format!("{}   {}  =>  {},", dep.name, dep.version, max);
        Some(line)
      } else {
        None
      }
    });

    let lines: Vec<String> = iter.collect();
    if lines.is_empty() {
      return;
    }

    println!(
      "[ {} ] {}",
      package
        .agent()
        .to_string()
        .to_uppercase()
        .bright_magenta()
        .bold(),
      package.name.bright_yellow().bold()
    );

    for line in lines {
      println!("  {line}");
    }
  }
}
