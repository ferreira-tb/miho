# Miho

Easily manage your project version.

```sh
cargo install miho
```

## CLI

```sh
miho bump [OPTIONS]
```

|      Command       | Description                                               |
| :----------------: | :-------------------------------------------------------- |
|      `--add`       | Include untracked files with `git add <PATHSPEC>`.        |
| `--commit-message` | Message of the commit, if any.                            |
|     `--no-ask`     | Do not ask for consent before bumping.                    |
|   `--no-commit`    | Do not commit the modified files.                         |
|    `--no-push`     | Do not push the commit.                                   |
|   `--no-verify`    | Bypass `pre-commit` and `commit-msg` hooks.               |
|     `--pre-id`     | Prerelease identifier, like the `beta` in `1.0.0-beta.1`. |
|  `--release-type`  | Type of the release.                                      |
|     `--stdio`      | Describes what to do with the standard I/O stream.        |

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
