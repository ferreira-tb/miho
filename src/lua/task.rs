use anyhow::Result;
use std::mem;
use tokio::process::Command;

#[derive(Debug)]
pub struct Task {
  commands: Vec<Vec<String>>,
}

impl Task {
  pub fn new<T: AsRef<str>>(task: T) -> Self {
    let mut commands = Vec::new();
    let mut command = Vec::new();

    for word in task.as_ref().split_whitespace() {
      if word == "&&" {
        let cmd = mem::take(&mut command);
        if !cmd.is_empty() {
          commands.push(cmd);
        }
      } else {
        command.push(word.to_owned());
      }
    }

    if !command.is_empty() {
      commands.push(command);
    }

    Self { commands }
  }

  pub async fn run(self) -> Result<()> {
    for command in self.commands {
      let Some((program, args)) = command.split_first() else {
        continue;
      };

      Command::new(program).args(args).spawn()?.wait().await?;
    }

    Ok(())
  }
}
