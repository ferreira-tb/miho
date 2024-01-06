/// Wrap [std::process::Command], executing `cmd` as the program if the current OS is Windows.
///
/// This is only useful in some very specific cases.
/// Prefer the standard library version.
#[macro_export]
macro_rules! win_cmd {
  ($program:literal) => {{
    let mut cmd = match std::env::consts::OS {
      "windows" => std::process::Command::new("cmd"),
      _ => std::process::Command::new($program),
    };

    if std::env::consts::OS == "windows" {
      cmd.arg("/C").arg($program);
    };

    cmd
  }};
}

#[macro_export]
macro_rules! gh {
  ($( $arg:literal ),*) => {{
    let mut args: Vec<&str> = Vec::new();
    $( args.push($arg); )*

    std::process::Command::new("gh")
      .args(args)
      .stderr(std::process::Stdio::piped())
      .stdout(std::process::Stdio::piped())
      .output()
  }};
}

#[macro_export]
macro_rules! pnpm {
  ($( $arg:literal ),*) => {{
    let mut args: Vec<&str> = Vec::new();
    $( args.push($arg); )*

    win_cmd!("pnpm")
      .args(args)
      .stderr(std::process::Stdio::inherit())
      .stdout(std::process::Stdio::inherit())
      .output()
  }};
  ($args:expr) => {{
    win_cmd!("pnpm")
      .args($args)
      .stderr(std::process::Stdio::inherit())
      .stdout(std::process::Stdio::inherit())
      .output()
  }};
}
