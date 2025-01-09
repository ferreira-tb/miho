mod bump;
mod update;

use anyhow::{Context, Result};
pub use bump::Bump;
use serde::Deserialize;
use std::env::current_dir;
use strum::{Display, EnumIter, EnumString};
use tokio::fs;
pub use update::Update;

const CONFIG_FILE: &str = "miho.toml";

pub trait Command {
  async fn execute(self, config: Option<Config>) -> Result<()>;
  fn merge(&mut self, value: &mut Self);
}

trait Commit: Command {
  async fn commit(&mut self, default_message: &str) -> Result<()>;
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct Config {
  bump: Bump,
  update: Update,
}

impl Config {
  pub async fn load() -> Result<Option<Self>> {
    let config: Result<Option<Self>> = try {
      let file = current_dir()?.join(CONFIG_FILE);
      if fs::try_exists(&file).await? {
        let contents = fs::read_to_string(&file).await?;
        let config = toml::from_str(&contents)?;
        Some(config)
      } else {
        None
      }
    };

    config.with_context(|| "failed to load config file")
  }
}

#[derive(Display, EnumIter, EnumString)]
#[strum(serialize_all = "title_case")]
enum Choice {
  All,
  Some,
  None,
}

enum PromptResult {
  Continue,
  Abort,
}
