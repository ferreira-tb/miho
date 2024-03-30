use crate::prelude::*;
use strum::EnumIs;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, PartialEq, Eq, EnumIs)]
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
  pub fn is_stable(&self) -> bool {
    self.is_major() || self.is_minor() || self.is_patch()
  }
}

#[derive(Default)]
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

  pub fn prerelease(&mut self, prerelease: &str) -> Result<&mut Self> {
    self.prerelease = Prerelease::new(prerelease)?;
    Ok(self)
  }

  pub fn metadata(&mut self, metadata: &str) -> Result<&mut Self> {
    self.metadata = BuildMetadata::new(metadata)?;
    Ok(self)
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
