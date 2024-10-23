# Miho

Easily bump your packages and update your dependencies.

```sh
rustup toolchain install nightly
cargo install miho
```

## Commands

### Bump

```sh
miho bump [OPTIONS] [RELEASE]
```

|      Options       | Alias | Description                                        |
| :----------------: | :---- | :------------------------------------------------- |
|      `--add`       | `-a`  | Include untracked files with `git add <PATHSPEC>`. |
|     `--agent`      | `-A`  | Only bump packages with the specified agents.      |
|     `--build`      | none  | Build metadata.                                    |
| `--commit-message` | `-m`  | Message of the commit.                             |
|    `--dry-run`     | `-d`  | Show what would be bumped.                         |
|     `--no-ask`     | `-k`  | Do not ask for consent before bumping.             |
|   `--no-commit`    | `-t`  | Do not commit the modified files.                  |
|    `--no-push`     | none  | Do not push the commit.                            |
|   `--no-verify`    | `-n`  | Bypass `pre-commit` and `commit-msg` hooks.        |
|    `--package`     | `-P`  | Packages to bump.                                  |
|      `--path`      | `-p`  | Where to search for packages.                      |
|      `--pre`       | none  | Prerelease identifier, e.g. `1.0.0-beta.1`.        |

### Update

```sh
miho update [OPTIONS] [RELEASE]
```

|       Options       | Alias | Description                                        |
| :-----------------: | :---- | :------------------------------------------------- |
|       `--add`       | `-a`  | Include untracked files with `git add <PATHSPEC>`. |
|      `--agent`      | `-A`  | Only update packages with the specified agents.    |
| `--commit-message`  | `-m`  | Message of the commit.                             |
|   `--dependency`    | `-D`  | Dependencies to update.                            |
|     `--dry-run`     | `-d`  | Show what would be updated.                        |
|     `--global`      | `-g`  | Update global dependencies.                        |
|     `--no-ask`      | `-k`  | Do not ask for consent before updating.            |
|    `--no-commit`    | `-t`  | Do not commit the modified files.                  |
|     `--no-push`     | none  | Do not push the commit.                            |
|    `--no-verify`    | `-n`  | Bypass `pre-commit` and `commit-msg` hooks.        |
|     `--package`     | `-P`  | Packages to update.                                |
|      `--path`       | `-p`  | Where to search for packages.                      |
|      `--peer`       | none  | Whether to only update peer dependencies.          |
| `--skip-dependency` | `-S`  | Skip updating dependencies.                        |

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
