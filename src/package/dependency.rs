use super::agent::Agent;
use crate::bail;
use crate::error::Error;
use reqwest::Client;
use semver::VersionReq;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;

const NPM_REGISTRY: &str = "https://registry.npmjs.org";

#[derive(Clone, Debug)]
pub enum DependencyKind {
  Normal,
  Dev,
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

  pub fn add(&mut self, dependencies: &HashMap<String, String>, kind: DependencyKind) -> &mut Self {
    for (name, version) in dependencies {
      let Ok(requirement) = VersionReq::parse(version) else {
        continue;
      };

      let dependency = Dependency {
        name: name.to_owned(),
        requirement,
        kind: kind.clone(),
        versions: Vec::default(),
      };

      self.dependencies.push(dependency);
    }

    self
  }

  pub async fn build(mut self) -> crate::Result<DependencyTree> {
    let mut set = JoinSet::new();
    let client = Client::builder().gzip(true).build()?;
    let client = Arc::new(client);

    for mut dep in &mut self.dependencies.drain(..) {
      let agent = self.agent.clone();
      let client = Arc::clone(&client);

      set.spawn(async move {
        let versions: Vec<String> = match agent {
          Agent::Cargo => bail!(Error::Unimplemented),

          // https://github.com/npm/registry/blob/master/docs/responses/package-metadata.md
          Agent::Npm | Agent::Pnpm | Agent::Yarn => {
            let url = format!("{NPM_REGISTRY}/{}", dep.name);
            let response = client
              .get(&url)
              .header("Accept", "application/vnd.npm.install-v1+json")
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
struct NpmResponse {
  versions: HashMap<String, serde_json::Value>,
}
