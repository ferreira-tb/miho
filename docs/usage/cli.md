# CLI

|               Command               | Alias | Description                                                         |
| :---------------------------------: | :---- | :------------------------------------------------------------------ |
|       [`--ask`](./cli.md#ask)       | none  | Determines whether Miho should ask for confirmation before bumping. |
| [`--recursive`](./cli.md#recursive) | `-r`  | Recursively bumps all packages in the monorepo.                     |
|    [`--ignore`](./cli.md#ignore)    | none  | Package names to ignore. May be regex.                              |
|   [`--exclude`](./cli.md#exclude)   | `-x`  | Glob patterns indicating where to not search for packages.          |
| [`--overrides`](./cli.md#overrides) | `-o`  | Allow to configure each package individually.                       |
|     [`--preid`](./cli.md#preid)     | `-p`  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.           |

The first positional argument will always be taken as the desired release type. It can be an arbitrary string containing a valid version, an integer or one of the constants described below:

- major
- premajor
- minor
- preminor
- patch
- prepatch

If the value of the argument is a version, Miho will bump the packages to that version. If the value is a number, it will perform a major bump to the version to which the number corresponds. Finally, using constants such as `major` and `patch` does what one would expect.

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

## Commands

### `--ask`

After getting the packages and being ready to bump them, Miho, by default, checks that you agree with the changes. When multiple packages are being bumped at the same time, Miho also allows you to specify which ones.

You can adjust this behavior using the `--no-ask` command. This way, Miho won't ask for your confirmation and will bump immediately.

```bash
npx miho patch --no-ask
```

#### `--recursive`

Recursively searches for packages in the directory and all its subdirectories, except `.git` and `node_modules`. To refine the search, use it together with other commands, such as [`--exclude`](https://github.com/ferreira-tb/miho#--exclude).

```bash
npx miho major -r
```

#### `--ignore`

Defines package names that should be ignored. Strings in the format `/abc/` will be treated as regex.

```bash
npx miho patch -r --ignore my-project /onlytest/
```

#### `--exclude`

Glob pattern indicating where Miho should not look for packages.

```bash
npx miho patch -r -x testdir/**
```

#### `--overrides`

Allows each package to be configured individually. Note that it is more appropriate to use a [config file](../index.md#config-file) in cases like this.

```bash
npx miho premajor -p beta -r -o.test=patch
```

#### `--preid`

Prerelease identifier. Only relevant when the release type is `premajor`, `preminor` or `prepatch`.

```bash
npx miho preminor -p alpha
```
