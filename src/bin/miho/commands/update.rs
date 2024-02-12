use crate::util::search_packages;
use clap::Args;
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

impl Update {
  pub async fn execute(&self) -> anyhow::Result<()> {
    let path = self.path.as_deref().unwrap_or_default();
    let packages = search_packages(path)?;

    let mut set = JoinSet::new();
    let trees = Vec::with_capacity(packages.len());
    let trees = Arc::new(Mutex::new(trees));

    for package in packages {
      let tree_vec = Arc::clone(&trees);
      set.spawn(async move {
        let tree = package.dependency_tree().await;
        let mut tree_vec = tree_vec.lock().unwrap();
        tree_vec.push((package, tree));
      });
    }

    while let Some(result) = set.join_next().await {
      result?;
    }

    let trees = Arc::into_inner(trees).unwrap().into_inner().unwrap();
    for (package, tree) in trees {
      println!("Updating package: {}", package.name);
      for dep in tree?.dependencies {
        println!("  - {} ({:?})", dep.name, dep.requirement);
      }
    }

    Ok(())
  }
}
