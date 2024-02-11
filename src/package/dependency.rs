use semver::VersionReq;
use std::collections::HashMap;

pub struct Dependency<'a> {
  pub name: &'a str,
  pub version: VersionReq,
}

impl<'a> TryFrom<(&'a str, &'a str)> for Dependency<'a> {
  type Error = crate::error::Error;

  fn try_from((name, version): (&'a str, &'a str)) -> crate::Result<Self> {
    let dep = Self {
      name,
      version: VersionReq::parse(version)?,
    };

    Ok(dep)
  }
}

#[derive(Default)]
pub struct DependencyTree<'a> {
  pub normal: Option<Vec<Dependency<'a>>>,
  pub dev: Option<Vec<Dependency<'a>>>,
  pub peer: Option<Vec<Dependency<'a>>>,
}

impl<'a> DependencyTree<'a> {
  pub fn builder() -> DependencyTreeBuilder<'a> {
    DependencyTreeBuilder::new()
  }
}

pub struct DependencyTreeBuilder<'a> {
  tree: DependencyTree<'a>,
}

impl<'a> DependencyTreeBuilder<'a> {
  pub fn new() -> Self {
    Self {
      tree: DependencyTree::default(),
    }
  }

  pub fn build(self) -> DependencyTree<'a> {
    self.tree
  }

  pub fn dev(&mut self, deps: &HashMap<String, String>) -> &mut Self {
    unimplemented!()
  }

  pub fn normal(&mut self, deps: &HashMap<String, String>) -> &mut Self {
    unimplemented!()
  }

  pub fn peer(&mut self, deps: &HashMap<String, String>) -> &mut Self {
    unimplemented!()
  }
}
