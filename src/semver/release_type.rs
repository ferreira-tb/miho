use anyhow::{anyhow, Result};

#[derive(Clone, Debug)]
pub enum ReleaseType {
  Major,
  Minor,
  Patch,
  PreMajor,
  PreMinor,
  PrePatch,
  PreRelease,
  Literal(String),
}

impl TryFrom<&str> for ReleaseType {
  type Error = anyhow::Error;

  fn try_from(val: &str) -> Result<Self> {
    let release_type = val.to_lowercase();
    let release_type = match release_type.trim() {
      "major" => ReleaseType::Major,
      "minor" => ReleaseType::Minor,
      "patch" => ReleaseType::Patch,
      "premajor" => ReleaseType::PreMajor,
      "preminor" => ReleaseType::PreMinor,
      "prepatch" => ReleaseType::PrePatch,
      "prerelease" => ReleaseType::PreRelease,
      rt => {
        if super::is_valid(rt) {
          ReleaseType::Literal(rt.to_string())
        } else {
          return Err(anyhow!("Cannot convert {rt} into a release type."));
        }
      }
    };

    Ok(release_type)
  }
}
