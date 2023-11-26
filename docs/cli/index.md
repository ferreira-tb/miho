---
outline: [2, 3]
---

# CLI

|                 Command                 | Alias   | Description                                                    |
| :-------------------------------------: | :------ | :------------------------------------------------------------- |
|             [`--all`](#all)             | `-a`    | Commit all modified files, not only the packages.              |
|             [`--ask`](#ask)             | none    | Whether Miho should ask for confirmation before bumping.       |
|           [`--build`](#build)           | `-b`    | Build the project.                                             |
|          [`--commit`](#commit)          | `-c`    | Commit the modified packages.                                  |
|         [`--dry-run`](#dry-run)         | `--dry` | Skip all jobs.                                                 |
|         [`--exclude`](#exclude)         | `-x`    | Glob patterns indicating where to **NOT** search for packages. |
|          [`--filter`](#filter)          | `-f`    | Package names to filter. May be regex.                         |
|            [`--help`](#help)            | `-h`    | Show usage information.                                        |
|         [`--include`](#include)         | `-i`    | Glob patterns indicating where to search for packages.         |
|       [`--no-verify`](#no-verify)       | `-n`    | Bypass `pre-commit` and `commit-msg` hooks.                    |
|            [`--only`](#only)            | `-l`    | Execute only one job.                                          |
|       [`--overrides`](#overrides)       | `-o`    | Allow to configure each package individually.                  |
| [`--package-manager`](#package-manager) | `--pm`  | Package manager being used.                                    |
|           [`--preid`](#preid)           | none    | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.      |
|         [`--publish`](#publish)         | none    | Publish the package.                                           |
|            [`--push`](#push)            | `-p`    | Push the commit.                                               |
|       [`--recursive`](#recursive)       | `-r`    | Recursively bumps all packages in the monorepo.                |
|          [`--silent`](#silent)          | none    | Omit unimportant logs.                                         |
|            [`--skip`](#skip)            | `-s`    | Skip one or more jobs.                                         |
|            [`--test`](#test)            | `-t`    | Run tests.                                                     |
|         [`--verbose`](#verbose)         | none    | Log additional info. May be useful for debugging.              |
|         [`--version`](#version)         | `-v`    | Show current version.                                          |

## Release

The first positional argument will always be taken as the desired release version or type. Possible values are:

- A valid semver version number
- A integer
- `major`
- `premajor`
- `minor`
- `preminor`
- `patch`
- `prepatch`

If it is a version, Miho will bump the packages to that specific version. If a integer, it will perform a major bump to the version it corresponds to. Finally, using constants such as `major` and `patch` does what one would expect.

Given a package whose version is `1.0.0`:

```bash
npx miho major
2.0.0
```

```bash
npx miho 17.23.12
17.23.12
```

```bash
npx miho 8
8.0.0
```

::: tip Default value
Miho will default to `patch` if you not specify a release type.
:::

## Pipeline

You can leverage Miho to configure a simple yet efficient pipeline for your project. For this purpose, commands like [`--build`](#build) and [`--publish`](#publish) can be used.

By default, the order of execution is (left to right):

`bump` => `build` => `test` => `commit` => `publish`

## Commands

### `--all`

| Alias |  Usage  |
| :---- | :-----: |
| `-a`  | `--all` |

Commit all modififed files, not only the packages. See [`git-commit`](https://git-scm.com/docs/git-commit#Documentation/git-commit.txt--a) for details.

You can omit [`-c`](#commit) if a custom message is not needed.

```bash
npx miho patch -a
```

### `--ask`

After getting the packages and being ready to bump them, Miho, by default, checks that you agree with the changes. When multiple packages are being bumped at the same time, Miho also allows you to specify which ones.

You can adjust this behavior using the `--no-ask` command. This way, Miho won't ask for your confirmation and will bump immediately.

```bash
npx miho patch --no-ask
```

### `--build`

| Alias |   Usage   |
| :---- | :-------: |
| `-b`  | `--build` |

After the `bump` job, run the `build` script defined in the root `package.json`.

You can provide a custom function using the [config file](../index.md#config-file).

```bash
npx miho patch -r -b -c "looks good"
```

### `--commit`

| Alias |        Usage         |
| :---- | :------------------: |
| `-c`  | `--commit [message]` |

After the `test` job, commit the modified packages.

If omitted, the message defaults to `chore: bump version`.

```bash
npx miho patch -c "commit message"
```

### `--dry-run`

| Alias   |    Usage    |
| :------ | :---------: |
| `--dry` | `--dry-run` |

Skip all jobs.

```bash
npx miho --dry
```

### `--exclude`

| Alias |          Usage           |
| :---- | :----------------------: |
| `-x`  | `--exclude [patterns..]` |

Glob patterns indicating where Miho should **not** look for packages.

```bash
npx miho patch -r -x foo/**
```

### `--filter`

| Alias |        Usage         |
| :---- | :------------------: |
| `-f`  | `--filter [names..]` |

Package names that should be filtered. Strings in the format `/abc/` will be treated as regex.

```bash
npx miho patch -r -f foo /bar/
```

### `--help`

| Alias |  Usage   |
| :---- | :------: |
| `-h`  | `--help` |

Show usage information.

### `--include`

| Alias |          Usage           |
| :---- | :----------------------: |
| `-i`  | `--include [patterns..]` |

Glob patterns indicating where to search for packages. By default, Miho will search the [current working directory](https://nodejs.org/dist/latest/docs/api/process.html#processcwd) (and also subdirectories, if [`--recursive`](#recursive)).

```bash
npx miho major -r -i foo/**
```

### `--no-verify`

| Alias |     Usage     |
| :---- | :-----------: |
| `-n`  | `--no-verify` |

By default, the [`pre-commit`](https://git-scm.com/docs/githooks#_pre_commit) and [`commit-msg`](https://git-scm.com/docs/githooks#_commit_msg) hooks are run. When any of `--no-verify` or `-n` is given, these are bypassed. See [`git-commit`](https://git-scm.com/docs/git-commit#Documentation/git-commit.txt--n) for details.

### `--only`

| Alias |     Usage      |
| :---- | :------------: |
| none  | `--only <job>` |

Execute only one job.

Possible value is one of those used for [`--skip`](#skip).

```bash
npx miho --only build
```

### `--overrides`

| Alias |               Usage               |
| :---- | :-------------------------------: |
| `-o`  | `--overrides.<package>=<release>` |

Allows each package to be configured individually. Note that it is more appropriate to use a [config file](../index.md#config-file) in cases like this.

```bash
npx miho premajor -p beta -r -o.foo=patch
```

### `--package-manager`

| Alias  |           Usage            |
| :----- | :------------------------: |
| `--pm` | `--package-manager <name>` |

Package manager being used. Defaults to `npm`.

If omitted, Miho will try to guess the package manager by looking at the `packageManager` key of the root `package.json`. If no such key is found, it will try to search for lock files, like `package-lock.json`, `pnpm-lock.json` and `yarn.lock`.

### `--preid`

| Alias |      Usage       |
| :---- | :--------------: |
| none  | `--preid <name>` |

Prerelease identifier. Must be used with `premajor`, `preminor` or `prepatch`.

```bash
npx miho preminor --preid alpha
```

### `--publish`

| Alias |    Usage    |
| :---- | :---------: |
| none  | `--publish` |

After the `commit` job, execute the `publish` command (e.g. [`npm publish`](https://docs.npmjs.com/cli/v8/commands/npm-publish)).

Miho is aware of your package manager and will adapt accordingly. However, you can also explicitly define it using the [`--package-manager`](#package-manager) command.

You can provide a custom function using the [config file](../index.md#config-file).

### `--push`

| Alias |  Usage   |
| :---- | :------: |
| `-p`  | `--push` |

Push the commit. See [`git-push`](https://git-scm.com/docs/git-push) for details.

```bash
npx miho -a -p
```

### `--recursive`

| Alias |     Usage     |
| :---- | :-----------: |
| `-r`  | `--recursive` |

Recursively searches for packages in the directory and all its subdirectories, except `.git` and `node_modules`. To refine the search, use it together with other commands, such as [`--exclude`](#exclude).

```bash
npx miho major -r
```

::: warning
If the search is not recursive, this option is ignored. Miho will only search the current directory.
:::

### `--silent`

| Alias |   Usage    |
| :---- | :--------: |
| none  | `--silent` |

Omit unimportant logs. Takes precedence over [`--verbose`](#verbose).

```bash
npx miho major -r --silent
```

### `--skip`

| Alias |       Usage       |
| :---- | :---------------: |
| `-s`  | `--skip [jobs..]` |

Skip one or more jobs.

Possible values are `build`, `bump`, `commit`, `publish` and `test`.

```bash
npx miho patch -s commit publish
```

### `--test`

| Alias |  Usage   |
| :---- | :------: |
| `-t`  | `--test` |

After the `build` job, run the `test` script defined in the root `package.json`.

You can provide a custom function using the [config file](../index.md#config-file).

### `--verbose`

| Alias |    Usage    |
| :---- | :---------: |
| none  | `--verbose` |

Log additional info. May be useful for debugging.

```bash
npx miho patch -r --verbose
```

### `--version`

| Alias |    Usage    |
| :---- | :---------: |
| `-v`  | `--version` |

Show current version.
