pub enum Flag {
  All,
  Message,
  NoVerify,
  Porcelain,
}

impl From<Flag> for &str {
  fn from(flag: Flag) -> Self {
    match flag {
      Flag::All => "--all",
      Flag::Message => "--message",
      Flag::NoVerify => "--no-verify",
      Flag::Porcelain => "--porcelain",
    }
  }
}

impl From<Flag> for String {
  fn from(flag: Flag) -> Self {
    let raw: &str = flag.into();
    String::from(raw)
  }
}
