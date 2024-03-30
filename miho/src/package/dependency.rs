use crate::package::agent::Agent;
use crate::prelude::*;
use crate::release::Release;
use crate::return_if_ne;
use crate::version::{ComparatorExt, VersionExt, VersionReqExt};
use reqwest::header::ACCEPT;
use reqwest::Client;
use serde_json::Value;
use strum::{AsRefStr, Display, EnumIs, EnumString};

const CARGO_REGISTRY: &str = "https://crates.io/api/v1/crates";
const NPM_REGISTRY: &str = "https://registry.npmjs.org";

#[derive(Debug)]
pub struct Dependency {
  pub name: String,
  pub comparator: Comparator,
  pub kind: DependencyKind,
  versions: Vec<Version>,
}

impl Dependency {
  #[must_use]
  pub fn latest(&self) -> Option<&Version> {
    self
      .versions
      .iter()
      .max_by(|a, b| Version::cmp_precedence(a, b))
  }

  #[must_use]
  pub fn latest_with_req(&self, requirement: &VersionReq) -> Option<&Version> {
    self
      .versions
      .iter()
      .filter(|v| requirement.matches_any(v))
      .max_by(|a, b| Version::cmp_precedence(a, b))
  }

  #[must_use]
  pub fn target_cmp(&self, release: &Option<Release>) -> Option<Comparator> {
    let comparator = &self.comparator;
    let requirement = if let Some(r) = release {
      comparator.with_release(r).as_version_req()
    } else {
      comparator.as_version_req()
    };

    self.latest_with_req(&requirement).and_then(|target| {
      let target_cmp = target.as_comparator(comparator.op);
      (target_cmp != *comparator).then_some(target_cmp)
    })
  }

  #[must_use]
  pub fn into_target(self, release: &Option<Release>) -> Option<Target> {
    let comparator = self.target_cmp(release);
    matches!(comparator, Some(ref t) if *t != self.comparator).then(|| Target {
      dependency: self,
      comparator: comparator.unwrap(),
    })
  }
}

impl PartialEq for Dependency {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.comparator == other.comparator && self.kind == other.kind
  }
}

impl Eq for Dependency {}

impl PartialOrd for Dependency {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Dependency {
  fn cmp(&self, other: &Self) -> Ordering {
    return_if_ne!(self.kind.cmp(&other.kind));
    self.name.cmp(&other.name)
  }
}

#[derive(Debug)]
pub struct DependencyTree {
  pub agent: Agent,
  pub dependencies: Vec<Dependency>,
}

impl DependencyTree {
  #[must_use]
  pub fn new(agent: Agent) -> Self {
    Self {
      agent,
      dependencies: Vec::default(),
    }
  }

  /// Adds dependencies to the tree.
  pub fn add<K, V>(&mut self, dependencies: &HashMap<K, V>, kind: DependencyKind) -> &mut Self
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
  pub async fn fetch(&mut self) -> Result<()> {
    let client = Client::builder()
      .user_agent("Miho/5.0")
      .brotli(true)
      .gzip(true)
      .build()?;

    let mut set = JoinSet::new();

    for mut dependency in &mut self.dependencies.drain(..) {
      let agent = self.agent.clone();
      let client = client.clone();

      set.spawn(async move {
        dependency.versions = match agent {
          // https://doc.rust-lang.org/cargo/reference/registry-web-api.html
          Agent::Cargo => {
            let url = format!("{CARGO_REGISTRY}/{}/versions", dependency.name);
            let response = client.get(&url).send().await?;

            let json: Value = response.json().await?;
            let Some(versions) = json.get("versions").and_then(Value::as_array) else {
              bail!("no versions found for {}", dependency.name);
            };

            versions
              .iter()
              .filter_map(|v| {
                let version = v.get("num").and_then(Value::as_str);
                version.and_then(|v| Version::parse(v).ok())
              })
              .collect()
          }

          // https://github.com/npm/registry/blob/master/docs/responses/package-metadata.md
          Agent::Npm | Agent::Pnpm => {
            let url = format!("{NPM_REGISTRY}/{}", dependency.name);
            let response = client
              .get(&url)
              .header(ACCEPT, "application/vnd.npm.install-v1+json")
              .send()
              .await?;

            let json: Value = response.json().await?;
            let Some(versions) = json.get("versions").and_then(Value::as_object) else {
              bail!("no versions found for {}", dependency.name);
            };

            versions
              .keys()
              .filter_map(|v| Version::parse(v).ok())
              .collect()
          }

          Agent::Tauri => bail!("tauri is not a package manager"),
        };

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

#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr, Display, EnumIs, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum DependencyKind {
  Build,
  #[strum(to_string = "dev")]
  Development,
  #[strum(to_string = "")]
  Normal,
  Peer,
}

impl DependencyKind {
  fn precedence(self) -> u8 {
    match self {
      Self::Normal => 0,
      Self::Development => 1,
      Self::Build => 2,
      Self::Peer => 3,
    }
  }
}

impl PartialOrd for DependencyKind {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for DependencyKind {
  fn cmp(&self, other: &Self) -> Ordering {
    self.precedence().cmp(&other.precedence())
  }
}

#[derive(Debug)]
pub struct Target {
  pub dependency: Dependency,
  pub comparator: Comparator,
}
