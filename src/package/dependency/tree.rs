use super::{Dependency, Kind};
use crate::package::agent::Agent;
use crate::version::{Comparator, Version};
use anyhow::{bail, Result};
use reqwest::header::ACCEPT;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;

const CARGO_REGISTRY: &str = "https://crates.io/api/v1/crates";
const NPM_REGISTRY: &str = "https://registry.npmjs.org";

#[derive(Debug)]
pub struct Tree {
  pub agent: Agent,
  pub dependencies: Vec<Dependency>,
}

impl Tree {
  #[must_use]
  pub fn new(agent: Agent) -> Self {
    Self {
      agent,
      dependencies: Vec::default(),
    }
  }

  /// Adds dependencies to the tree.
  pub fn add<K, V>(&mut self, dependencies: &HashMap<K, V>, kind: Kind) -> &mut Self
  where
    K: AsRef<str>,
    V: AsRef<str>,
  {
    for (name, version) in dependencies {
      let version = version.as_ref();
      let Ok(comparator) = Comparator::parse(version) else {
        continue;
      };

      let dependency = Dependency {
        name: name.as_ref().to_owned(),
        comparator,
        kind,
        versions: Vec::default(),
      };

      self.dependencies.push(dependency);
    }

    self
  }

  /// Updates the dependency tree, fetching metadata from the registry.
  pub async fn fetch_metadata(&mut self) -> Result<()> {
    let client = Client::builder()
      .user_agent("Miho/4.1")
      .brotli(true)
      .gzip(true)
      .build()?;

    let client = Arc::new(client);
    let mut set = JoinSet::new();

    for mut dependency in &mut self.dependencies.drain(..) {
      let agent = self.agent.clone();
      let client = Arc::clone(&client);

      set.spawn(async move {
        let versions: Vec<String> = match agent {
          // https://doc.rust-lang.org/cargo/reference/registry-web-api.html
          Agent::Cargo => {
            let url = format!("{CARGO_REGISTRY}/{}/versions", dependency.name);
            let response = client.get(&url).send().await?;

            let json: CargoResponse = response.json().await?;
            json.versions.into_iter().map(|v| v.num).collect()
          }

          // https://github.com/npm/registry/blob/master/docs/responses/package-metadata.md
          Agent::Npm | Agent::Pnpm | Agent::Yarn => {
            let url = format!("{NPM_REGISTRY}/{}", dependency.name);
            let response = client
              .get(&url)
              .header(ACCEPT, "application/vnd.npm.install-v1+json")
              .send()
              .await?;

            let json: NpmResponse = response.json().await?;
            json.versions.into_keys().collect()
          }

          Agent::Tauri => bail!("tauri is not a package manager"),
        };

        dependency.versions = versions
          .into_iter()
          .filter_map(|v| Version::parse(&v).ok())
          .collect();

        dependency.versions.shrink_to_fit();

        Ok(dependency)
      });
    }

    while let Some(result) = set.join_next().await {
      let dependency = result??;
      if !dependency.versions.is_empty() {
        self.dependencies.push(dependency);
      }
    }

    self.dependencies.shrink_to_fit();

    Ok(())
  }
}

#[derive(Deserialize)]
struct CargoResponse {
  versions: Vec<CargoVersion>,
}

#[derive(Deserialize)]
struct CargoVersion {
  num: String,
}

#[derive(Deserialize)]
struct NpmResponse {
  versions: HashMap<String, serde_json::Value>,
}
