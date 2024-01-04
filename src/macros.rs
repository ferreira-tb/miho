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