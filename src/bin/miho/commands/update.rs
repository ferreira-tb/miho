use crate::search_packages;
use clap::Args;
use tokio::task::JoinSet;

#[derive(Debug, Args)]
pub struct Update {
  /// Where to search for packages.
  #[arg(short = 'i', long = "include", value_name = "PATH")]
  paths: Option<Vec<String>>,
}

impl Update {
  pub async fn execute(&self) -> anyhow::Result<()> {
    let paths = self.paths.as_deref().unwrap_or_default();
    let packages = search_packages(paths)?;

    let mut set = JoinSet::new();

    for package in packages {
      set.spawn(async move { package.dependency_tree().await });
    }

    while let Some(result) = set.join_next().await {
      let _ = result??;
    }

    Ok(())
  }
}
