mod task;

use crate::match_once;
use anyhow::{anyhow, Result};
use std::path::Path;
use std::{env, fs};
pub use task::Task;
use tokio::task::JoinSet;

pub struct Lua {
  lua: mlua::Lua,
}

impl Lua {
  const MIHO: &'static str = "miho";
  const TASK: &'static str = "task";

  pub fn new<C: AsRef<str>>(chunk: C) -> Result<Self> {
    let lua = mlua::Lua::new();

    let miho = lua.create_table()?;
    lua.globals().set(Self::MIHO, miho)?;

    lua.load(chunk.as_ref()).exec()?;

    Ok(Self { lua })
  }

  pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
    let path = path.as_ref();

    let chunk = if path.is_absolute() {
      fs::read_to_string(path)?
    } else {
      let path = env::current_dir()?.join(path);
      fs::read_to_string(path)?
    };

    Self::new(chunk)
  }

  pub async fn run_task(&self, task: &str, parallel: bool) -> Result<()> {
    let tasks = self.collect_tasks(task)?;
    if tasks.is_empty() {
      return Err(anyhow!("no such task: {task}"));
    }

    let mut set = JoinSet::new();

    for task in tasks {
      if parallel {
        set.spawn(task.run());
      } else {
        task.run().await?;
      }
    }

    while let Some(result) = set.join_next().await {
      result??;
    }

    Ok(())
  }

  fn collect_tasks(&self, task: &str) -> Result<Vec<Task>> {
    let mut tasks = Vec::new();
    let Ok(mut table) = self.miho()?.get::<_, mlua::Table>(Self::TASK) else {
      return Ok(tasks);
    };

    let mut iter = task.split(':').peekable();

    while let Some(part) = iter.next() {
      // Treats each part before the last as a table.
      if iter.peek().is_some() {
        table = table.get::<_, mlua::Table>(part)?;
      } else {
        let value: mlua::Value = table.get(part)?;

        // If the last part evaluates to a string, collect the task.
        if value.is_string() {
          tasks.push(Task::new(value.as_str().unwrap()));

        // If it evaluates to a table, collect all tasks within it.
        } else if value.is_table() {
          let value = value.as_table().unwrap();
          value.for_each(|key: mlua::Value, _: mlua::Value| -> mlua::Result<()> {
            match_once!(key, mlua::Value::String(key) => {
              let mut inner_task = String::from(task);
                inner_task.push_str(&format!(":{}", key.to_str()?));

                if let Ok(inner_tasks) = self.collect_tasks(&inner_task) {
                  tasks.extend(inner_tasks);
                }
            });

            Ok(())
          })?;
        }
      }
    }

    Ok(tasks)
  }

  fn miho(&self) -> Result<mlua::Table> {
    let miho: mlua::Table = self.lua.globals().get(Self::MIHO)?;
    Ok(miho)
  }
}
