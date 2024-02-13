use crate::version::Version;

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
