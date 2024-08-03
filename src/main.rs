#![feature(let_chains, try_blocks)]

mod agent;
mod command;
mod dependency;
mod git;
mod macros;
mod package;
mod prelude;
mod release;
mod version;

use clap::Parser;
use command::{Bump, Command, Update};
use prelude::*;

#[derive(Debug, Parser)]
#[command(name = "miho")]
#[command(version, about, long_about = None)]
enum Cli {
  /// Bump your packages version.
  Bump(Bump),
  /// Update your dependencies.
  Update(Update),
}

#[tokio::main]
async fn main() -> Result<()> {
  #[cfg(feature = "tracing")]
  setup_tracing()?;

  match Cli::parse() {
    Cli::Bump(cmd) => cmd.execute().await,
    Cli::Update(cmd) => cmd.execute().await,
  }
}

#[cfg(feature = "tracing")]
fn setup_tracing() -> Result<()> {
  use tracing::subscriber::set_global_default;
  use tracing_subscriber::fmt::time::ChronoLocal;
  use tracing_subscriber::fmt::Layer;
  use tracing_subscriber::layer::SubscriberExt;
  use tracing_subscriber::{EnvFilter, Registry};

  /// <https://docs.rs/chrono/latest/chrono/format/strftime/index.html>
  const TIMESTAMP: &str = "%F %T%.3f %:z";

  let filter = EnvFilter::builder()
    .from_env()?
    .add_directive("miho=trace".parse()?);

  let stderr = Layer::default()
    .with_ansi(true)
    .with_timer(ChronoLocal::new(TIMESTAMP.into()))
    .with_writer(std::io::stderr)
    .pretty();

  let subscriber = Registry::default().with(stderr).with(filter);

  set_global_default(subscriber).map_err(Into::into)
}
