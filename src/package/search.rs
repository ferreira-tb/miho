use crate::agent::Agent;
use crate::package::manifest::ManifestKind;
use crate::package::Package;
use crate::prelude::*;
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::{DirEntry, WalkBuilder};
use std::path::Path;

#[derive(Debug)]
pub struct SearchBuilder<'a> {
  path: Vec<&'a Path>,
  packages: Vec<&'a str>,
  agents: Vec<Agent>,
}

impl<'a> SearchBuilder<'a> {
  pub fn new(path: &[&'a Path]) -> Self {
    Self {
      path: path.to_vec(),
      packages: Vec::new(),
      agents: Vec::new(),
    }
  }

  pub fn package(mut self, name: &[&'a str]) -> Self {
    self.packages.extend(name);
    self
  }

  pub fn agent(mut self, agents: &[&str]) -> Self {
    let agents = agents
      .iter()
      .map(|it| it.to_uppercase())
      .filter_map(|it| Agent::try_from(it.as_str()).ok())
      .unique();

    self.agents.extend(agents);
    self
  }

  #[cfg_attr(feature = "tracing", instrument)]
  pub fn search(self) -> Result<Vec<Package>> {
    #[cfg(feature = "tracing")]
    info!("searching packages in: {:?}", self.path);

    let Some((first, other)) = self.path.split_first() else {
      return Ok(Vec::new());
    };

    let mut walker = WalkBuilder::new(first);
    for path in other {
      walker.add(path);
    }

    let glob = build_globset()?;
    let mut packages = Vec::new();

    for entry in walker.build().flatten() {
      if is_match(&glob, &entry) {
        let path = entry.path().canonicalize()?;
        let package = Package::new(path);
        if matches!(package, Ok(ref it) if !packages.contains(it)) {
          let package = package.unwrap();

          #[cfg(feature = "tracing")]
          info!("found: {:?}", package.path.display());

          packages.push(package);
        }
      }
    }

    if !self.packages.is_empty() {
      #[cfg(feature = "tracing")]
      info!("keeping packages with name: {:?}", self.packages);

      packages.retain(|it| self.packages.contains(&it.name.as_str()));
    }

    if !self.agents.is_empty() {
      #[cfg(feature = "tracing")]
      info!("keeping packages with agent: {:?}", self.agents);

      packages.retain(|it| self.agents.contains(&it.manifest.agent()));
    }

    if packages.is_empty() {
      bail!("no valid package found");
    }

    packages.sort_unstable();

    Ok(packages)
  }
}

fn build_globset() -> Result<GlobSet> {
  let mut builder = GlobSetBuilder::new();

  macro_rules! add {
    ($kind:ident) => {
      let glob = ManifestKind::$kind.glob();
      builder.add(Glob::new(glob)?);
    };
  }

  add!(CargoToml);
  add!(PackageJson);
  add!(TauriConfJson);

  builder.build().map_err(Into::into)
}

fn is_match(glob: &GlobSet, entry: &DirEntry) -> bool {
  if !glob.is_match(entry.path()) {
    return false;
  }

  matches!(entry.file_type(), Some(it) if !it.is_dir())
}
