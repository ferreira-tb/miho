mod release_type;
mod version;

use regex::Regex;
pub use release_type::ReleaseType;
pub use version::Version;

/// <https://regex101.com/r/VX7uQk>
pub const SEMVER_REGEX: &str =
  r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([a-zA-Z-]+)(?:\.(\d+)))?$";

/// Whether the slice is a version accepted by Miho.
pub fn is_valid<V: AsRef<str>>(version: V) -> bool {
  let version = version.as_ref();
  let regex = Regex::new(SEMVER_REGEX).unwrap();
  regex.is_match(version)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn should_be_valid() {
    let versions = ["1.0.0", "0.2.3-beta.1"];
    for version in versions {
      assert!(is_valid(version));
    }
  }

  #[test]
  fn should_be_invalid() {
    let versions = ["1", "3.5", "1.0.0-beta", "2.0.0-rc.1+build.123"];
    for version in versions {
      assert!(!is_valid(version));
    }
  }

  #[test]
  fn should_build_version_struct() {
    let version = Version::new("6.2.3-beta.1").unwrap();
    assert_eq!(6, version.major);
    assert_eq!(2, version.minor);
    assert_eq!(3, version.patch);
    assert_eq!("beta", version.pre_id.unwrap());
    assert_eq!(1, version.pre_version.unwrap());
  }
}
