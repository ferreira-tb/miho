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
  ($command:expr) => {{
    let path = $command
      .path
      .as_deref()
      .expect("should have `.` as the default value")
      .iter()
      .map(|it| it.as_path())
      .collect_vec();

    let mut builder = $crate::package::SearchBuilder::new(&path);
    if let Some(packages) = $command.package.as_deref() {
      let packages = packages
        .iter()
        .map(|it| it.as_str())
        .collect_vec();

      builder = builder.package(&packages);
    }

    if let Some(agents) = $command.agent.as_deref() {
      let agents = agents.iter().map(|it| it.as_str()).collect_vec();
      builder = builder.agent(&agents);
    }

    builder.search()?
  }};
}
