use crate::util::search_packages;
use anyhow::Result;
use clap::Args;
use colored::Colorize;
use miho::package::dependency;
use miho::package::Package;
use miho::Release;
use std::sync::{Arc, Mutex};
use tokio::task::JoinSet;

type DependencyTree = (Package, dependency::Tree);

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

impl Update {
  pub async fn execute(&self) -> Result<()> {
    let path = self.path.as_deref().unwrap_or_default();
    let packages = search_packages(path)?;

    if packages.is_empty() {
      println!("{}", "No valid package found.".bold().red());
      return Ok(());
    }

    let release = self.release();
    let trees = self.fetch_trees(packages).await?;

    for (package, tree) in trees {
      println!("Updating package: {}", package.name);

      /*if let Some(release) = release {
        unimplemented!()
      } else {
        unimplemented!()
      }*/
    }

    Ok(())
  }

  async fn fetch_trees(&self, packages: Vec<Package>) -> Result<Vec<DependencyTree>> {
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
      .map(|r| r.try_into())
      .transpose()
      .unwrap_or(None)
  }
}