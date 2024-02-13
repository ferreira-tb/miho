use super::agent::Agent;
use crate::bail;
use crate::error::Error;
use reqwest::header::ACCEPT;
use reqwest::Client;
use semver::VersionReq;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;

const CARGO_REGISTRY: &str = "https://crates.io/api/v1/crates";
const NPM_REGISTRY: &str = "https://registry.npmjs.org";

#[derive(Clone, Debug)]
pub enum DependencyKind {
  Build,
  Development,
  Normal,
  Peer,
}

#[derive(Debug)]
pub struct Dependency {
  pub name: String,
  pub requirement: VersionReq,
  pub kind: DependencyKind,
  pub versions: Vec<String>,
}

#[derive(Debug)]
pub struct DependencyTree {
  pub agent: Agent,
  pub dependencies: Vec<Dependency>,
}

impl DependencyTree {
  pub fn builder(agent: Agent) -> DependencyTreeBuilder {
    DependencyTreeBuilder::new(agent)
  }
}

pub struct DependencyTreeBuilder {
  agent: Agent,
  dependencies: Vec<Dependency>,
}

impl DependencyTreeBuilder {
  pub fn new(agent: Agent) -> Self {
    Self {
      agent,
      dependencies: Vec::default(),
    }
  }

  /// Adds a list of dependencies to the tree.
  pub fn add<K, V>(&mut self, dependencies: &HashMap<K, V>, kind: DependencyKind) -> &mut Self
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
        kind: kind.clone(),
        versions: Vec::default(),
      };

      self.dependencies.push(dependency);
    }

    self
  }

  /// Builds the dependency tree, fetching metadata from their respective registries.
  pub async fn build(mut self) -> crate::Result<DependencyTree> {
    let client = Client::builder()
      .user_agent("Miho/4.0")
      .brotli(true)
      .gzip(true)
      .build()?;

    let client = Arc::new(client);
    let mut set = JoinSet::new();

    for mut dep in &mut self.dependencies.drain(..) {
      let agent = self.agent.clone();
      let client = Arc::clone(&client);

      set.spawn(async move {
        let versions: Vec<String> = match agent {
          // https://doc.rust-lang.org/cargo/reference/registry-web-api.html
          Agent::Cargo => {
            let url = format!("{CARGO_REGISTRY}/{}/versions", dep.name);
            let response = client.get(&url).send().await?;

            let json: CargoResponse = response.json().await?;
            json.versions.into_iter().map(|v| v.num).collect()
          }

          // https://github.com/npm/registry/blob/master/docs/responses/package-metadata.md
          Agent::Npm | Agent::Pnpm | Agent::Yarn => {
            let url = format!("{NPM_REGISTRY}/{}", dep.name);
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

        dep.versions = versions;

        Ok(dep)
      });
    }

    while let Some(result) = set.join_next().await {
      let dep = result??;
      self.dependencies.push(dep);
    }

    self.dependencies.shrink_to_fit();

    let tree = DependencyTree {
      agent: self.agent,
      dependencies: self.dependencies,
    };

    Ok(tree)
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