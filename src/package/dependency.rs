use super::agent::Agent;
use crate::error::Error;
use crate::version::{Version, VersionReq};
use crate::{bail, Result};
use reqwest::header::ACCEPT;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::task::JoinSet;

const CARGO_REGISTRY: &str = "https://crates.io/api/v1/crates";
const NPM_REGISTRY: &str = "https://registry.npmjs.org";

#[derive(Clone, Copy, Debug)]
pub enum Kind {
  Build,
  Development,
  Normal,
  Peer,
}

impl fmt::Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Build => write!(f, "build"),
      Self::Development => write!(f, "dev"),
      Self::Normal => write!(f, ""),
      Self::Peer => write!(f, "peer"),
    }
  }
}

#[derive(Debug)]
pub struct Dependency {
  pub name: String,
  pub requirement: VersionReq,
  pub kind: Kind,

  /// Available versions that satisfy the requirement.
  ///
  /// This field is empty until `fetch_metadata` is called.
  pub versions: Vec<Version>,
}

impl Dependency {
  /// Returns the maximum version that satisfies the requirement.
  #[must_use]
  pub fn max_version(&self) -> Option<&Version> {
    let max = self
      .versions
      .iter()
      .max_by(|a, b| Version::cmp_precedence(a, b));

    if matches!(max, Some(m) if self.requirement.matches(m)) {
      max
    } else {
      None
    }
  }
}

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
      let Ok(requirement) = VersionReq::parse(version.as_ref()) else {
        continue;
      };

      let dependency = Dependency {
        name: name.as_ref().to_owned(),
        requirement,
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
      .user_agent("Miho/4.0")
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

          Agent::Tauri => bail!(Error::NotPackageManager),
        };

        let version = versions.into_iter().filter_map(|v| {
          let requirement = &dependency.requirement;
          Version::parse(&v).ok().filter(|v| requirement.matches(v))
        });

        dependency.versions = version.collect();
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
