use anyhow::{bail, Result};
use clap::Args;
use colored::Colorize;
use miho::lua::Lua;

#[derive(Debug, Args)]
pub struct Run {
  /// Tasks to run.
  task: Vec<String>,

  /// Path to the configuration file.
  #[arg(short = 'c', long, default_value = ".config/miho.lua")]
  config: Option<String>,

  /// Run tasks in parallel.
  #[arg(short = 'P', long)]
  parallel: bool,
}

impl super::Command for Run {
  async fn execute(self) -> Result<()> {
    if self.task.is_empty() {
      bail!("{}", "No task to run.".bold().red());
    }

    let lua = Lua::from_path(self.config.as_deref().unwrap())?;
    for task in self.task {
      lua.run_task(&task, self.parallel).await?;
    }

    Ok(())
  }
}
