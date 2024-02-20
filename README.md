# Miho

Bump your packages version, update your dependencies, and run tasks defined in a configuration file leveraging the power of [Lua](https://www.lua.org/start.html).

```sh
cargo install miho
```

## CLI

### Bump

Bump your packages version.

```sh
miho bump [OPTIONS] [RELEASE]
```

|      Options       | Alias | Description                                        |
| :----------------: | :---- | :------------------------------------------------- |
|      `--add`       | `-a`  | Include untracked files with `git add <PATHSPEC>`. |
|     `--build`      | none  | Build metadata.                                    |
| `--commit-message` | `-m`  | Message of the commit.                             |
|     `--no-ask`     | `-k`  | Do not ask for consent before bumping.             |
|   `--no-commit`    | none  | Do not commit the modified files.                  |
|    `--no-push`     | none  | Do not push the commit.                            |
|   `--no-verify`    | `-n`  | Bypass `pre-commit` and `commit-msg` hooks.        |
|    `--package`     | `-P`  | Packages to bump.                                  |
|      `--path`      | `-p`  | Where to search for packages.                      |
|      `--pre`       | none  | Prerelease identifier, e.g. `1.0.0-beta.1`.        |

### Run

Run one or more tasks defined in the `miho.lua` configuration file.

```sh
miho run [OPTIONS] [TASKS]...
```

|   Options    | Alias | Description                     |
| :----------: | :---- | :------------------------------ |
|  `--config`  | `-c`  | Path to the configuration file. |
| `--parallel` | `-P`  | Run the tasks in parallel.      |

Given the following configuration file:

```lua
-- .config/miho.lua

miho.task = {
  cargo = 'cargo --version',
  rustc = 'rustc --version',
}
```

You can run the `cargo` and `rustc` tasks with the command:

```sh
miho run cargo rustc
```

### Update

Update your dependencies.

```sh
miho update [OPTIONS] [RELEASE]
```

|   Options   | Alias | Description                             |
| :---------: | :---- | :-------------------------------------- |
| `--no-ask`  | `-k`  | Do not ask for consent before updating. |
| `--package` | `-P`  | Packages to update.                     |
|  `--path`   | `-p`  | Where to search for packages.           |
|  `--peer`   | none  | Whether to include peer dependencies.   |

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
