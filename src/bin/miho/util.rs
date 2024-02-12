use miho::{Package, SearchBuilder};

pub fn search_packages<P>(path: &[P]) -> anyhow::Result<Vec<Package>>
where
  P: AsRef<str>,
{
  let mut paths: Vec<&str> = path.iter().map(|g| g.as_ref()).collect();

  let last = paths.pop().unwrap_or(".");
  let mut builder = SearchBuilder::new(last);

  for path in paths {
    builder.add(path);
  }

  Ok(builder.search()?)
}
