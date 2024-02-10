use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct Update;

impl super::Command for Update {
  fn execute(&mut self) -> Result<()> {
    unimplemented!()
  }
}
