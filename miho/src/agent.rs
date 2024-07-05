use std::cmp::Ordering;
use strum::{AsRefStr, Display, EnumIs, EnumString};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, AsRefStr, Display, EnumString, EnumIs)]
#[strum(serialize_all = "UPPERCASE")]
pub enum Agent {
  Cargo,
  Npm,
  Pnpm,
  Tauri,
}

impl Agent {
  pub fn is_node(self) -> bool {
    self.is_npm() || self.is_pnpm()
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

impl PartialOrd for Agent {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Agent {
  fn cmp(&self, other: &Self) -> Ordering {
    let agent = self.as_ref();
    agent.cmp(other.as_ref())
  }
}
