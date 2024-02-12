# Miho

Repository management tools.

```sh
cargo install miho
```

## CLI

### Bump

Bump your packages version.

```sh
miho bump [OPTIONS] [RELEASE_TYPE]
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
|      `--path`      | `-p`  | Where to search for packages.                      |
|      `--pre`       | none  | Prerelease identifier, e.g. `1.0.0-beta.1`.        |

### Update

Update your dependencies.

```sh
miho update [OPTIONS]
```

|   Options   | Alias | Description                             |
| :---------: | :---- | :-------------------------------------- |
| `--install` | `-i`  | Install the updated packages.           |
| `--no-ask`  | `-k`  | Do not ask for consent before updating. |
|  `--path`   | `-p`  | Where to search for packages.           |

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
