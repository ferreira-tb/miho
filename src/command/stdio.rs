use std::process;

#[derive(Copy, Clone)]
pub enum Stdio {
  Null,
  Piped,
  Inherit,
}

impl Stdio {
  pub fn as_std_stdio(&self) -> process::Stdio {
    match self {
      Stdio::Null => process::Stdio::null(),
      Stdio::Piped => process::Stdio::piped(),
      Stdio::Inherit => process::Stdio::inherit(),
    }
  }
}

impl<T: AsRef<str>> From<T> for Stdio {
  fn from(val: T) -> Self {
    match val.as_ref() {
      "null" => Stdio::Null,
      "piped" => Stdio::Piped,
      _ => Stdio::Inherit,
    }
  }
}
