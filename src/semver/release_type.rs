use anyhow::{anyhow, Result};

#[derive(Copy, Clone, Debug)]
pub enum ReleaseType {
  Major,
  Minor,
  Patch,
  PreMajor,
  PreMinor,
  PrePatch,
  PreRelease,
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
      rt => return Err(anyhow!("Cannot convert {rt} into a release type.")),
    };

    Ok(release_type)
  }
}
