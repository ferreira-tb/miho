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
macro_rules! win_cmd {
  ($program:expr) => {{
    let mut cmd = if cfg!(windows) {
      tokio::process::Command::new("cmd")
    } else {
      tokio::process::Command::new($program)
    };

    if cfg!(windows) {
      cmd.arg("/C").arg($program);
    };

    cmd
  }};
}
