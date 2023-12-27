# Miho

> [!CAUTION] > **THIS PACKAGE IS DEPRECATED**
>
> Miho has been rewritten in Rust and is no longer available on NPM.
> Consider using the [new version](https://crates.io/crates/miho).

Easily manage your package.json version.

- Bump, build, test, commit and publish.
- Simple Javascript API.
- Easy to use CLI commands.

```bash
npm create miho@latest
```

## CLI

|                              Command                              | Alias   | Description                                                    |
| :---------------------------------------------------------------: | :------ | :------------------------------------------------------------- |
|             [`--all`](https://tb.dev.br/miho/cli#all)             | `-a`    | Commit all modified files, not only the packages.              |
|             [`--ask`](https://tb.dev.br/miho/cli#ask)             | none    | Whether Miho should ask for confirmation before bumping.       |
|           [`--build`](https://tb.dev.br/miho/cli#build)           | `-b`    | Build the project.                                             |
|          [`--commit`](https://tb.dev.br/miho/cli#commit)          | `-c`    | Commit the modified packages.                                  |
|         [`--dry-run`](https://tb.dev.br/miho/cli#dry-run)         | `--dry` | Skip all jobs.                                                 |
|         [`--exclude`](https://tb.dev.br/miho/cli#exclude)         | `-x`    | Glob patterns indicating where to **NOT** search for packages. |
|          [`--filter`](https://tb.dev.br/miho/cli#filter)          | `-f`    | Package names to filter. May be regex.                         |
|            [`--help`](https://tb.dev.br/miho/cli#help)            | `-h`    | Show usage information.                                        |
|         [`--include`](https://tb.dev.br/miho/cli#include)         | `-i`    | Glob patterns indicating where to search for packages.         |
|       [`--no-verify`](https://tb.dev.br/miho/cli#no-verify)       | `-n`    | Bypass `pre-commit` and `commit-msg` hooks.                    |
|            [`--only`](https://tb.dev.br/miho/cli#only)            | `-l`    | Execute only one job.                                          |
|       [`--overrides`](https://tb.dev.br/miho/cli#overrides)       | `-o`    | Allow to configure each package individually.                  |
| [`--package-manager`](https://tb.dev.br/miho/cli#package-manager) | `--pm`  | Package manager being used.                                    |
|           [`--preid`](https://tb.dev.br/miho/cli#preid)           | none    | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.      |
|         [`--publish`](https://tb.dev.br/miho/cli#publish)         | none    | Publish the package.                                           |
|            [`--push`](https://tb.dev.br/miho/cli#push)            | `-p`    | Push the commit.                                               |
|       [`--recursive`](https://tb.dev.br/miho/cli#recursive)       | `-r`    | Recursively bumps all packages in the monorepo.                |
|          [`--silent`](https://tb.dev.br/miho/cli#silent)          | none    | Omit unimportant logs.                                         |
|            [`--skip`](https://tb.dev.br/miho/cli#skip)            | `-s`    | Skip one or more jobs.                                         |
|            [`--test`](https://tb.dev.br/miho/cli#test)            | `-t`    | Run tests.                                                     |
|         [`--verbose`](https://tb.dev.br/miho/cli#verbose)         | none    | Log additional info. May be useful for debugging.              |
|         [`--version`](https://tb.dev.br/miho/cli#version)         | `-v`    | Show current version.                                          |

## Documentation

Read the [documentation](https://docs.rs/miho) for more details.

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
