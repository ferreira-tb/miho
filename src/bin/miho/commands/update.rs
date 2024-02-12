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

    for package in &packages {
      let builder = package.manifest.dependency_tree();
      set.spawn(builder.build());
    }

    while let Some(result) = set.join_next().await {
      let _ = result??;
    }

    Ok(())
  }
}
