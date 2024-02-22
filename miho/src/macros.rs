#[doc(hidden)]
#[macro_export]
macro_rules! git_spawn {
  ($command:expr, $args:expr) => {{
    let mut child = $command.args($args).spawn()?;
    let status = child.wait().await?;
    Ok(status)
  }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! git_output {
  ($command:expr, $args:expr) => {{
    let output = $command.args($args).output().await?;
    Ok(output)
  }};
}

/// Returns the [`std::cmp::Ordering`] if it is not [`std::cmp::Ordering::Equal`].
#[doc(hidden)]
#[macro_export]
macro_rules! return_if_ne {
  ($ord:expr) => {
    let ord = $ord;
    if ord != std::cmp::Ordering::Equal {
      return ord;
    }
  };
}

#[doc(hidden)]
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

/// Wrap [`tokio::process::Command`], executing `cmd` as the program if the current OS is Windows.
///
/// This is only useful in some very specific cases.
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
