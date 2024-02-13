use miho::package::builder::{Builder, Bump, Search};
use miho::package::Package;
use miho::Release;
use std::path::{Path, PathBuf};
use std::{env, fs};

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

#[test]
fn should_find_package() {
  let search = Search::new(".");
  let entries = search.execute().unwrap();
  let cwd = env::current_dir().unwrap();

  let toml = cwd.join("Cargo.toml").canonicalize().unwrap();
  let toml = toml.to_str().unwrap();

  if !entries.iter().any(|p| p.path.to_str().unwrap() == toml) {
    panic!("Cargo.toml not found");
  }
}

macro_rules! create_package {
  ($manifest:expr) => {{
    let mocks = find_mocks_dir(env::current_dir().unwrap()).unwrap();
    Package::new(mocks.join($manifest)).unwrap()
  }};
}

#[test]
fn should_create_package_from_cargo_toml() {
  let package = create_package!("Cargo.toml");
  assert_eq!(package.name, "cargo-toml");
  assert_eq!(package.filename(), "Cargo.toml");
}

#[test]
fn should_create_package_from_package_json() {
  let package = create_package!("package.json");
  assert_eq!(package.name, "package-json");
  assert_eq!(package.filename(), "package.json");
}

#[test]
fn should_create_package_from_tauri_conf_json() {
  let package = create_package!("tauri.conf.json");
  assert_eq!(package.name, "tauri-conf-json");
  assert_eq!(package.filename(), "tauri.conf.json");
}

macro_rules! bump {
  ($manifest:expr) => {
    let mocks = find_mocks_dir(env::current_dir().unwrap()).unwrap();
    let path = mocks.join($manifest);

    let package = Package::new(&path).unwrap();
    let current_patch = package.version.patch;

    let bump = Bump::new(package, &Release::Patch);
    bump.execute().unwrap();

    let package = Package::new(path).unwrap();
    assert_eq!(package.version.patch, current_patch + 1);
  };
}

#[test]
fn should_bump_cargo_toml() {
  bump!("Cargo.toml");
}

#[test]
fn should_bump_package_json() {
  bump!("package.json");
}

#[test]
fn should_bump_tauri_conf_json() {
  bump!("tauri.conf.json");
}
