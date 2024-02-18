miho.task = {
  clippy = 'cargo clippy -- -D warnings',
  format = 'cmd /C prettier . --write && cargo fmt',
  pedantic = 'cargo clippy -- -W clippy::pedantic',
  release = 'cargo test && cargo clippy && cargo publish',
}

miho.task.mock = {
  bump = 'cargo run -- bump -p mocks --no-commit',
  run = 'cargo run -- run inner:mock -c ./mocks/miho.lua -P',
  update = 'cargo run -- update -p mocks',
}
