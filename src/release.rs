use crate::version::{BuildMetadata, Prerelease, Version};
use crate::Result;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Release {
  Major(BuildMetadata),
  Minor(BuildMetadata),
  Patch(BuildMetadata),
  PreMajor(Prerelease, BuildMetadata),
  PreMinor(Prerelease, BuildMetadata),
  PrePatch(Prerelease, BuildMetadata),
  PreRelease(Prerelease, BuildMetadata),
  Literal(Version),
}

impl Release {
  #[must_use]
  pub fn parser() -> Parser {
    Parser::new()
  }

  #[must_use]
  pub fn is_literal(&self) -> bool {
    matches!(self, Release::Literal(_))
  }

  #[must_use]
  pub fn is_prerelease(&self) -> bool {
    !self.is_stable() && !self.is_literal()
  }

  #[must_use]
  pub fn is_stable(&self) -> bool {
    matches!(
      self,
      Release::Major(_) | Release::Minor(_) | Release::Patch(_)
    )
  }
}

pub struct Parser {
  prerelease: Prerelease,
  metadata: BuildMetadata,
}

impl Parser {
  #[must_use]
  fn new() -> Self {
    Self {
      prerelease: Prerelease::EMPTY,
      metadata: BuildMetadata::EMPTY,
    }
  }

  pub fn prerelease(&mut self, prerelease: Prerelease) -> &mut Self {
    self.prerelease = prerelease;
    self
  }

  pub fn metadata(&mut self, metadata: BuildMetadata) -> &mut Self {
    self.metadata = metadata;
    self
  }

  pub fn parse(self, release: &str) -> Result<Release> {
    let release = release.to_lowercase();
    let release = match release.trim() {
      "major" => Release::Major(self.metadata),
      "minor" => Release::Minor(self.metadata),
      "patch" => Release::Patch(self.metadata),
      "premajor" => Release::PreMajor(self.prerelease, self.metadata),
      "preminor" => Release::PreMinor(self.prerelease, self.metadata),
      "prepatch" => Release::PrePatch(self.prerelease, self.metadata),
      "prerelease" => Release::PreRelease(self.prerelease, self.metadata),
      rt => {
        let version = Version::parse(rt)?;
        Release::Literal(version)
      }
    };

    Ok(release)
  }
}

impl Default for Parser {
  fn default() -> Self {
    Self::new()
  }
}
