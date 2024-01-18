use anyhow::{bail, Result};

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
          bail!("cannot convert {rt} into a release type.");
        }
      }
    };

    Ok(release_type)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_release_type_from_str() {
    let release_type = ReleaseType::try_from("major").unwrap();
    assert_eq!(matches!(release_type, ReleaseType::Major), true);

    let release_type = ReleaseType::try_from("minor").unwrap();
    assert_eq!(matches!(release_type, ReleaseType::Minor), true);

    let release_type = ReleaseType::try_from("patch").unwrap();
    assert_eq!(matches!(release_type, ReleaseType::Patch), true);

    let release_type = ReleaseType::try_from("premajor").unwrap();
    assert_eq!(matches!(release_type, ReleaseType::PreMajor), true);

    let release_type = ReleaseType::try_from("preminor").unwrap();
    assert_eq!(matches!(release_type, ReleaseType::PreMinor), true);

    let release_type = ReleaseType::try_from("prepatch").unwrap();
    assert_eq!(matches!(release_type, ReleaseType::PrePatch), true);

    let release_type = ReleaseType::try_from("prerelease").unwrap();
    assert_eq!(matches!(release_type, ReleaseType::PreRelease), true);

    let release_type = ReleaseType::try_from("1.0.0").unwrap();
    if let ReleaseType::Literal(v) = release_type {
      assert_eq!(v, "1.0.0");
    } else {
      panic!("expected literal");
    }
  }
}
