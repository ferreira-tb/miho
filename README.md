# Miho

Repository management tools.

```sh
cargo install miho
```

## CLI

### Bump

Recursively bump your packages version.

```sh
miho bump [OPTIONS] [RELEASE_TYPE]
```

|      Options       | Alias | Description                                               |
| :----------------: | :---- | :-------------------------------------------------------- |
|      `--add`       | `-a`  | Include untracked files with `git add <PATHSPEC>`.        |
| `--commit-message` | `-m`  | Message of the commit.                                    |
|    `--include`     | `-i`  | Where to search for packages.                             |
|     `--no-ask`     | none  | Do not ask for consent before bumping.                    |
|   `--no-commit`    | none  | Do not commit the modified files.                         |
|    `--no-push`     | none  | Do not push the commit.                                   |
|   `--no-verify`    | `n`   | Bypass `pre-commit` and `commit-msg` hooks.               |
|     `--pre-id`     | none  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`. |
|     `--stdio`      | `-s`  | Describes what to do with the standard I/O stream.        |

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
