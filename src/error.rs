/// Returns early with an error.
#[doc(hidden)]
#[macro_export]
macro_rules! bail {
  ($err:expr) => {{
    return Err($err);
  }};
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("git command failed:\n{reason}")]
  Git { reason: String },

  #[error("path does not resolve to a valid manifest:\n{path}")]
  InvalidManifestPath { path: String },

  #[error("agent is not a package manager")]
  NotPackageManager,

  #[error("unimplemented")]
  Unimplemented,

  // Transparent errors
  #[error(transparent)]
  Io(#[from] std::io::Error),

  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),

  #[error(transparent)]
  Semver(#[from] semver::Error),

  #[error(transparent)]
  SerdeJson(#[from] serde_json::Error),

  #[error(transparent)]
  TokioJoin(#[from] tokio::task::JoinError),

  #[error(transparent)]
  TomlDe(#[from] toml::de::Error),

  #[error(transparent)]
  TomlSer(#[from] toml::ser::Error),
}
