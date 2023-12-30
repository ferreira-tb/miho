use miho::package;
use std::env;

#[test]
fn should_find_package() {
  let entries = package::search().unwrap();
  let cwd = env::current_dir().unwrap();
  let toml = cwd.join("Cargo.toml").canonicalize().unwrap();

  if !entries.iter().any(|p| p.to_str() == toml.to_str()) {
    panic!("Cargo.toml not found");
  }
}
