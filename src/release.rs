use crate::version::Version;

macro_rules! match_release {
  ( $release:expr, $( $variant:ident ),* ) => {{
    match $release {
      $( Release::$variant => true, )*
      _ => false
    }
  }};
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Release {
  Major,
  Minor,
  Patch,
  PreMajor,
  PreMinor,
  PrePatch,
  PreRelease,
  Literal(Version),
}

impl Release {
  #[must_use]
  pub fn is_literal(&self) -> bool {
    matches!(self, Release::Literal(_))
  }

  #[must_use]
  pub fn is_pre(&self) -> bool {
    match_release!(self, PreMajor, PreMinor, PrePatch, PreRelease)
  }

  #[must_use]
  pub fn is_stable(&self) -> bool {
    match_release!(self, Major, Minor, Patch)
  }
}

impl TryFrom<&str> for Release {
  type Error = crate::error::Error;

  fn try_from(val: &str) -> crate::Result<Self> {
    let release = val.to_lowercase();
    let release = match release.trim() {
      "major" => Release::Major,
      "minor" => Release::Minor,
      "patch" => Release::Patch,
      "premajor" => Release::PreMajor,
      "preminor" => Release::PreMinor,
      "prepatch" => Release::PrePatch,
      "prerelease" => Release::PreRelease,
      rt => {
        let version = Version::parse(rt)?;
        Release::Literal(version)
      }
    };

    Ok(release)
  }
}
