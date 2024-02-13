use super::Builder;
use crate::package::Package;
use crate::release::Release;
use semver::{BuildMetadata, Prerelease};

pub struct Bump<'a> {
  package: &'a Package,
  release: &'a Release,
  pre: Prerelease,
  build: BuildMetadata,
}

impl Builder for Bump<'_> {
  type Output = ();

  fn execute(self) -> crate::Result<Self::Output> {
    let mut new_version = if self.pre.is_empty() {
      self.release.increment(&self.package.version)
    } else {
      self.release.increment_pre(&self.package.version, self.pre)
    };

    if !self.build.is_empty() {
      new_version.build = self.build;
    }

    self.package.manifest.bump(self.package, new_version)
  }
}

impl<'a> Bump<'a> {
  #[must_use]
  pub fn new(package: &'a Package, release: &'a Release) -> Self {
    Self {
      package,
      release,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

  /// Sets the prerelease version.
  pub fn pre<P: AsRef<str>>(&mut self, pre: P) -> crate::Result<&mut Self> {
    self.pre = Prerelease::new(pre.as_ref())?;
    Ok(self)
  }

  /// Sets the build metadata.
  pub fn build<B: AsRef<str>>(&mut self, build: B) -> crate::Result<&mut Self> {
    self.build = BuildMetadata::new(build.as_ref())?;
    Ok(self)
  }
}
