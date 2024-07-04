# Miho

Easily bump your packages and update your dependencies.

```sh
cargo install miho
```

## CLI

### Bump

```sh
miho bump [OPTIONS] [RELEASE]
```

|      Options       | Alias | Description                                        |
| :----------------: | :---- | :------------------------------------------------- |
|      `--add`       | `-a`  | Include untracked files with `git add <PATHSPEC>`. |
|     `--build`      | none  | Build metadata.                                    |
| `--commit-message` | `-m`  | Message of the commit.                             |
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

|      Options       | Alias | Description                                        |
| :----------------: | :---- | :------------------------------------------------- |
|      `--add`       | `-a`  | Include untracked files with `git add <PATHSPEC>`. |
| `--commit-message` | `-m`  | Message of the commit.                             |
|   `--dependency`   | `-D`  | Dependencies to update.                            |
|     `--no-ask`     | `-k`  | Do not ask for consent before updating.            |
|   `--no-commit`    | `-t`  | Do not commit the modified files.                  |
|    `--no-push`     | none  | Do not push the commit.                            |
|   `--no-verify`    | `-n`  | Bypass `pre-commit` and `commit-msg` hooks.        |
|    `--package`     | `-P`  | Packages to update.                                |
|      `--path`      | `-p`  | Where to search for packages.                      |
|      `--peer`      | none  | Whether to only update peer dependencies.          |

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
