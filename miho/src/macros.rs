#[macro_export]
macro_rules! return_if_ne {
  ($ord:expr) => {
    let ord = $ord;
    if ord != std::cmp::Ordering::Equal {
      return ord;
    }
  };
}

#[macro_export]
macro_rules! command {
  ($program:expr) => {{
    #[cfg(target_os = "windows")]
    {
      let mut cmd = tokio::process::Command::new("cmd");
      cmd.args(&["/C", $program]);
      cmd
    }

    #[cfg(not(target_os = "windows"))]
    tokio::process::Command::new($program)
  }};
}
