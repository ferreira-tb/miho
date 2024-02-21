local publish = 'cargo publish -p miho_derive && cargo publish -p miho'

miho.task = {
  clippy = 'cargo clippy -- -D warnings',
  format = 'cmd /C prettier . --write && cargo fmt --all',
  pedantic = 'cargo clippy -- -W clippy::pedantic',
  release = 'cargo test && cargo clippy && ' .. publish,
}

miho.task.mock = {
  bump = 'cargo run -- bump -p mocks --no-commit',
  run = 'cargo run -- run inner:mock -c ./mocks/miho.lua -P',
  update = 'cargo run -- update major -p mocks -k',
}
