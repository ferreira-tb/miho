use miho::package::builder::{self, Builder};
use miho::package::Package;

pub fn search_packages<P>(path: &[P]) -> anyhow::Result<Vec<Package>>
where
  P: AsRef<str>,
{
  let mut paths: Vec<&str> = path.iter().map(|g| g.as_ref()).collect();

  let last = paths.pop().unwrap_or(".");
  let mut search = builder::Search::new(last);

  for path in paths {
    search.add(path);
  }

  Ok(search.execute()?)
}