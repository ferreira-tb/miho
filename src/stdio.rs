use std::process::Stdio;

#[derive(Copy, Clone)]
pub enum MihoStdio {
  Null,
  Piped,
  Inherit,
}

impl MihoStdio {
  pub fn as_stdio(&self) -> Stdio {
    match self {
      MihoStdio::Null => Stdio::null(),
      MihoStdio::Piped => Stdio::piped(),
      MihoStdio::Inherit => Stdio::inherit(),
    }
  }
}

impl<T: AsRef<str>> From<T> for MihoStdio {
  fn from(val: T) -> Self {
    match val.as_ref() {
      "null" => MihoStdio::Null,
      "piped" => MihoStdio::Piped,
      _ => MihoStdio::Inherit,
    }
  }
}
