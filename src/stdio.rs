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

impl From<&str> for MihoStdio {
  fn from(val: &str) -> Self {
    let string = val.to_lowercase();
    match string.as_str() {
      "null" => MihoStdio::Null,
      "piped" => MihoStdio::Piped,
      _ => MihoStdio::Inherit,
    }
  }
}

impl From<MihoStdio> for &str {
  fn from(item: MihoStdio) -> Self {
    match item {
      MihoStdio::Null => "null",
      MihoStdio::Piped => "piped",
      MihoStdio::Inherit => "inherit",
    }
  }
}

impl From<String> for MihoStdio {
  fn from(val: String) -> Self {
    let string = val.to_lowercase();
    string.as_str().into()
  }
}

impl From<MihoStdio> for String {
  fn from(item: MihoStdio) -> Self {
    let string: &str = item.into();
    string.to_string()
  }
}

impl From<&String> for MihoStdio {
  fn from(val: &String) -> Self {
    let string = val.to_lowercase();
    string.as_str().into()
  }
}

impl From<MihoStdio> for Stdio {
  fn from(item: MihoStdio) -> Self {
    match item {
      MihoStdio::Null => Self::null(),
      MihoStdio::Piped => Self::piped(),
      MihoStdio::Inherit => Self::inherit(),
    }
  }
}
