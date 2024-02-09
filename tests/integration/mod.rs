mod package;

use std::fs;
use std::path::{Path, PathBuf};

fn find_mocks_dir<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
  let path = path.as_ref();
  if path.is_dir() {
    for entry in fs::read_dir(path).unwrap() {
      let entry = entry.unwrap();
      if entry.file_name() == "mocks" && entry.file_type().unwrap().is_dir() {
        return Some(entry.path());
      }
    }

    return find_mocks_dir(path.parent().unwrap());
  }

  None
}
