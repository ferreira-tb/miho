use crate::agent::Agent;
use crate::release::Release;
use crate::return_if_ne;
use crate::version::{ComparatorExt, VersionExt, VersionReqExt};
use anyhow::{Result, bail};
use itertools::Itertools;
use reqwest::Client;
use reqwest::header::ACCEPT;
use semver::{Comparator, Version, VersionReq};
use serde_json::Value;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, LazyLock, Mutex};
use std::{fmt, mem};
use strum::{AsRefStr, Display, EnumIs, EnumString};
use tokio::task::JoinSet;

pub type Cache = HashSet<DependencyCache>;

const CARGO_REGISTRY: &str = "https://crates.io/api/v1/crates";
const NPM_REGISTRY: &str = "https://registry.npmjs.org";

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| {
  Client::builder()
    .use_rustls_tls()
    .user_agent(USER_AGENT)
    .brotli(true)
    .gzip(true)
    .build()
    .expect("failed to create http client")
});

#[derive(Debug)]
pub struct Dependency {
  pub name: String,
  pub comparator: Comparator,
  pub kind: DependencyKind,
  versions: Vec<Version>,
}

impl Dependency {
  pub fn latest(&self) -> Option<&Version> {
    self
      .versions
      .iter()
      .max_by(|a, b| Version::cmp_precedence(a, b))
  }

  pub fn latest_with_req(&self, requirement: &VersionReq) -> Option<&Version> {
    self
      .versions
      .iter()
      .filter(|v| requirement.matches_any(v))
      .max_by(|a, b| Version::cmp_precedence(a, b))
  }

  pub fn as_target(&self, release: Option<&Release>) -> Option<Target> {
    let comparator = &self.comparator;
    let requirement = if let Some(it) = release {
      comparator.with_release(it).as_version_req()
    } else {
      comparator.as_version_req()
    };

    let mut target_cmp = self
      .latest_with_req(&requirement)
      .and_then(|version| {
        let target_cmp = version.as_comparator(comparator.op);
        (target_cmp != *comparator).then_some(target_cmp)
      })?;

    comparator.normalize(&mut target_cmp);

    if target_cmp == *comparator {
      None
    } else {
      Some(Target::new(self, target_cmp))
    }
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
pub struct DependencyCache {
  pub agent: Agent,
  pub name: String,
  pub versions: Vec<Version>,
}

impl PartialEq for DependencyCache {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name && self.agent == other.agent
  }
}

impl Eq for DependencyCache {}

impl Hash for DependencyCache {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.name.hash(state);
    self.agent.hash(state);
  }
}

#[derive(Debug)]
pub struct DependencyTree {
  pub agent: Agent,
  pub dependencies: Vec<Dependency>,
}

impl DependencyTree {
  pub fn new(agent: Agent) -> Self {
    Self { agent, dependencies: Vec::new() }
  }

  pub fn add(&mut self, name: impl AsRef<str>, comparator: Comparator, kind: DependencyKind) {
    let dependency = Dependency {
      name: name.as_ref().to_owned(),
      comparator,
      kind,
      versions: Vec::new(),
    };

    self.dependencies.push(dependency);
  }

  /// Add dependencies to the tree.
  pub fn add_many<I, N, V>(&mut self, dependencies: I, kind: DependencyKind)
  where
    I: IntoIterator<Item = (N, V)>,
    N: AsRef<str>,
    V: AsRef<str>,
  {
    for (name, version) in dependencies {
      let version = version.as_ref();
      if let Ok(comparator) = Comparator::parse(version) {
        self.add(name, comparator, kind);
      }
    }
  }

  /// Update the dependency tree, fetching metadata from the registry.
  pub async fn fetch(&mut self, cache: Arc<Mutex<Cache>>) -> Result<()> {
    let mut set = JoinSet::new();

    let dependencies = mem::take(&mut self.dependencies);
    self.dependencies.reserve(dependencies.len());

    for mut dependency in dependencies {
      let agent = self.agent;
      let cache = Arc::clone(&cache);

      {
        let cache = cache.lock().unwrap();
        if let Some(cached) = Self::find_cached(&cache, &dependency.name, agent) {
          dependency.versions.clone_from(&cached.versions);
          self.dependencies.push(dependency);
          continue;
        }
      }

      set.spawn(async move {
        dependency.versions = match agent {
          Agent::Cargo => Self::fetch_cargo(&dependency, agent, cache).await?,
          Agent::Npm | Agent::Pnpm => Self::fetch_npm(&dependency, agent, cache).await?,
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

  /// <https://doc.rust-lang.org/cargo/reference/registry-web-api.html>
  async fn fetch_cargo(
    dependency: &Dependency,
    agent: Agent,
    cache: Arc<Mutex<Cache>>,
  ) -> Result<Vec<Version>> {
    let url = format!("{CARGO_REGISTRY}/{}/versions", dependency.name);
    let response = HTTP_CLIENT.get(&url).send().await?;

    let json: Value = response.json().await?;
    let Some(versions) = json.get("versions").and_then(Value::as_array) else {
      bail!("no versions found for {}", dependency.name);
    };

    let versions = versions
      .iter()
      .filter_map(Self::parse_cargo_version)
      .collect_vec();

    let mut cache = cache.lock().unwrap();
    Self::add_to_cache(&mut cache, &dependency.name, agent, &versions);

    Ok(versions)
  }

  fn parse_cargo_version(version: &Value) -> Option<Version> {
    if version
      .get("yanked")
      .and_then(Value::as_bool)
      .eq(&Some(true))
    {
      return None;
    }

    version
      .get("num")
      .and_then(Value::as_str)
      .and_then(|it| Version::parse(it).ok())
  }

  /// <https://github.com/npm/registry/blob/master/docs/responses/package-metadata.md>
  async fn fetch_npm(
    dependency: &Dependency,
    agent: Agent,
    cache: Arc<Mutex<Cache>>,
  ) -> Result<Vec<Version>> {
    let url = format!("{NPM_REGISTRY}/{}", dependency.name);
    let response = HTTP_CLIENT
      .get(&url)
      .header(ACCEPT, "application/vnd.npm.install-v1+json")
      .send()
      .await?;

    let json: Value = response.json().await?;
    let Some(versions) = json.get("versions").and_then(Value::as_object) else {
      bail!("no versions found for {}", dependency.name);
    };

    let versions = versions
      .values()
      .filter_map(Self::parse_npm_version)
      .collect_vec();

    let mut cache = cache.lock().unwrap();
    Self::add_to_cache(&mut cache, &dependency.name, agent, &versions);

    Ok(versions)
  }

  fn parse_npm_version(version: &Value) -> Option<Version> {
    if version
      .get("deprecated")
      .and_then(Value::as_str)
      .is_some_and(|it| !it.is_empty())
    {
      return None;
    }

    version
      .get("version")
      .and_then(Value::as_str)
      .and_then(|it| Version::parse(it).ok())
  }

  fn add_to_cache(cache: &mut Cache, name: &str, agent: Agent, versions: &[Version]) {
    if Self::find_cached(cache, name, agent).is_none() {
      let dependency = DependencyCache {
        agent,
        name: name.to_owned(),
        versions: versions.to_vec(),
      };

      cache.insert(dependency);
    }
  }

  fn find_cached<'a>(cache: &'a Cache, name: &str, agent: Agent) -> Option<&'a DependencyCache> {
    cache
      .iter()
      .find(|c| c.name == name && c.agent == agent)
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, AsRefStr, Display, EnumIs, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub enum DependencyKind {
  Build,
  #[strum(to_string = "dev")]
  Development,
  #[strum(to_string = "")]
  Normal,
  Peer,
  PackageManager,
}

impl DependencyKind {
  fn precedence(self) -> u8 {
    match self {
      DependencyKind::Normal => 0,
      DependencyKind::Development => 1,
      DependencyKind::Build => 2,
      DependencyKind::Peer => 3,
      DependencyKind::PackageManager => 4,
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
pub struct Target<'a> {
  pub dependency: &'a Dependency,
  pub comparator: Comparator,
}

impl<'a> Target<'a> {
  pub fn new(dependency: &'a Dependency, comparator: Comparator) -> Self {
    Self { dependency, comparator }
  }
}

impl fmt::Display for Target<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.comparator)
  }
}
