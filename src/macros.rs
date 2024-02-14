/// Returns early with an error.
#[doc(hidden)]
#[macro_export]
macro_rules! bail {
  ($err:expr) => {{
    return Err($err);
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
