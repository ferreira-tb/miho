use std::cmp::Ordering;
use std::fmt;

/// Agent responsible for the manifest.
///
/// This tipically represents the package manager used.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Agent {
  Cargo,
  Npm,
  Pnpm,
  Tauri,
}

impl Agent {
  pub fn is_cargo(&self) -> bool {
    *self == Agent::Cargo
  }

  pub fn is_node(&self) -> bool {
    matches!(self, Agent::Npm | Agent::Pnpm)
  }

  pub fn is_tauri(&self) -> bool {
    *self == Agent::Tauri
  }

  pub fn lockfile(&self) -> Option<&str> {
    match self {
      Self::Cargo => Some("Cargo.lock"),
      Self::Npm => Some("package-lock.json"),
      Self::Pnpm => Some("pnpm-lock.yaml"),
      Self::Tauri => None,
    }
  }
}

impl From<Agent> for &str {
  fn from(agent: Agent) -> Self {
    match agent {
      Agent::Cargo => "cargo",
      Agent::Npm => "npm",
      Agent::Pnpm => "pnpm",
      Agent::Tauri => "tauri",
    }
  }
}

impl fmt::Display for Agent {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let agent: &str = self.clone().into();
    write!(f, "{agent}")
  }
}

impl PartialOrd for Agent {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Agent {
  fn cmp(&self, other: &Self) -> Ordering {
    let first: &str = self.clone().into();
    let second: &str = other.clone().into();

    first.cmp(second)
  }
}
