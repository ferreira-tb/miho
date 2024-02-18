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
macro_rules! match_once {
  ($expr:expr, $pat:pat => $then:expr) => {
    match $expr {
      $pat => $then,
      _ => {}
    }
  };
}
