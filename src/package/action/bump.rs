use super::Action;
use crate::package::Package;
use crate::release::Release;
use crate::version::{BuildMetadata, Prerelease, VersionExt};
use crate::Result;

pub struct Bump<'a> {
  package: Package,
  release: &'a Release,
  pre: Prerelease,
  build: BuildMetadata,
}

impl Action for Bump<'_> {
  type Output = Result<()>;

  fn execute(self) -> Self::Output {
    let mut new_version = if self.pre.is_empty() {
      self.package.version.inc(self.release)
    } else {
      self.package.version.inc_with_pre(self.release, self.pre)
    };

    if !self.build.is_empty() {
      new_version.build = self.build;
    }

    self.package.manifest.bump(&self.package, new_version)
  }
}

impl<'a> Bump<'a> {
  #[must_use]
  pub fn new(package: Package, release: &'a Release) -> Self {
    Self {
      package,
      release,
      pre: Prerelease::EMPTY,
      build: BuildMetadata::EMPTY,
    }
  }

  /// Sets the prerelease version.
  pub fn pre<P: AsRef<str>>(&mut self, pre: P) -> Result<&mut Self> {
    self.pre = Prerelease::new(pre.as_ref())?;
    Ok(self)
  }

  /// Sets the build metadata.
  pub fn build<B: AsRef<str>>(&mut self, build: B) -> Result<&mut Self> {
    self.build = BuildMetadata::new(build.as_ref())?;
    Ok(self)
  }
}
