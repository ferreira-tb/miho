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
    use itertools::Itertools;

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

#[macro_export]
macro_rules! impl_commit {
  ($name:ident) => {
    impl $crate::command::Commit for $name {
      async fn commit(&mut self, default_message: &str) -> Result<()> {
        use $crate::git::{self, Git};

        if let Some(pathspec) = &self.add {
          git::Add::new(pathspec).spawn().await?;
        }

        let message = if !self.no_ask && self.commit_message.is_none() {
          inquire::Text::new("Commit message: ").prompt_skippable()?
        } else {
          self.commit_message.take()
        };

        let message = match message.as_deref().map(str::trim) {
          Some(m) if !m.is_empty() => m,
          _ => default_message,
        };

        let mut commit = git::Commit::new(message);
        commit.all();

        if self.no_verify {
          commit.no_verify();
        }

        commit.spawn().await?;

        if !self.no_push {
          git::Push::new().spawn().await?;
        }

        Ok(())
      }
    }
  };
}
