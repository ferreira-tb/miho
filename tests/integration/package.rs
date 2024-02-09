use super::find_mocks_dir;
use miho::{BumpBuilder, Package, Release, SearchBuilder};
use std::env;

#[test]
fn should_find_package() {
  let builder = SearchBuilder::new(".");
  let entries = builder.search().unwrap();
  let cwd = env::current_dir().unwrap();

  let toml = cwd.join("Cargo.toml").canonicalize().unwrap();
  let toml = toml.to_str().unwrap();

  if !entries
    .iter()
    .any(|p| p.manifest_path.to_str().unwrap() == toml)
  {
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

    let builder = BumpBuilder::new(&package, &Release::Patch);
    builder.bump().unwrap();

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
