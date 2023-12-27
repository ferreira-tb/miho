# Miho

Easily manage your project version.

```bash
cargo install miho
```

## CLI

|     Command      | Description                                               |
| :--------------: | :-------------------------------------------------------- |
|    `--commit`    | Commit the modified packages.                             |
|     `--help`     | Show usage information.                                   |
|    `--no-ask`    | Do not ask for consent before bumping.                    |
|   `--no-push`    | Do not push the commit.                                   |
|  `--no-verify`   | Bypass `pre-commit` and `commit-msg` hooks.               |
|    `--pre-id`    | Prerelease identifier, like the `beta` in `1.0.0-beta.1`. |
|  `--recursive`   | Recursively bumps all packages in the monorepo.           |
| `--release-type` | Type of the release.                                      |
|    `--stdio`     | Describes what to do with the standard I/O stream.        |
|   `--version`    | Show current version.                                     |

## Documentation

Read the [documentation](https://tb.dev.br/miho) for more details.

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
