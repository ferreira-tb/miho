local run = 'cargo run -- '

miho.task = {
  clippy = 'cargo clippy -- -D warnings',
  format = 'cmd /C prettier . --write && cargo fmt',
  pedantic = 'cargo clippy -- -W clippy::pedantic',
  release = 'cargo test && cargo clippy && cargo publish',
}

miho.task.mock = {
  bump = run .. 'bump -p mocks --no-commit',
  run = run .. 'run inner:mock -c ./mocks/miho.lua -P',
  update = run .. 'update -p mocks',
}
