use std::cmp::Ordering;
use std::fmt;

/// Agent responsible for the manifest.
///
/// This tipically represents the package manager used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Agent {
  Cargo,
  Npm,
  Pnpm,
  Tauri,
  Yarn,
}

impl From<Agent> for &str {
  fn from(agent: Agent) -> Self {
    match agent {
      Agent::Cargo => "cargo",
      Agent::Npm => "npm",
      Agent::Pnpm => "pnpm",
      Agent::Tauri => "tauri",
      Agent::Yarn => "yarn",
    }
  }
}

impl fmt::Display for Agent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let agent: &str = self.clone().into();
    write!(f, "{agent}")
  }
}

impl Ord for Agent {
  fn cmp(&self, other: &Self) -> Ordering {
    let first: &str = self.clone().into();
    let second: &str = other.clone().into();

    first.cmp(second)
  }
}

impl PartialOrd for Agent {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
