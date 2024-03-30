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
macro_rules! search_packages {
  ($path:expr) => {{
    $crate::package::Package::search($path)
  }};

  ($path:expr, $names:expr) => {{
    let names = $names;
    $crate::package::Package::search($path).map(|mut packages| {
      if matches!(names, Some(n) if !n.is_empty()) {
        let names = names.unwrap();
        packages.retain(|package| names.contains(&package.name));
      }

      packages
    })
  }};
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
