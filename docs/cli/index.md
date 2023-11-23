---
outline: [2, 3]
---

# CLI

|           Command           | Alias | Description                                                         |
| :-------------------------: | :---- | :------------------------------------------------------------------ |
|       [`--ask`](#ask)       | none  | Determines whether Miho should ask for confirmation before bumping. |
| [`--recursive`](#recursive) | `-r`  | Recursively bumps all packages in the monorepo.                     |
|   [`--include`](#include)   | `-i`  | Glob pattern indicating where to search for packages.               |
|   [`--exclude`](#exclude)   | `-x`  | Glob patterns indicating where to **NOT** search for packages.      |
|    [`--filter`](#filter)    | `-f`  | Package names to filter. May be regex.                              |
| [`--overrides`](#overrides) | `-o`  | Allow to configure each package individually.                       |
|     [`--preid`](#preid)     | `-p`  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.           |

The first positional argument will always be taken as the desired release type. It can be an arbitrary string containing a valid version, an integer or one of the constants below:

- major
- premajor
- minor
- preminor
- patch
- prepatch

If it is a version, Miho will bump the packages to that version. If a number, it will perform a major bump to the version it corresponds to. Finally, using constants such as `major` and `patch` does what one would expect.

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

### `--recursive`

Recursively searches for packages in the directory and all its subdirectories, except `.git` and `node_modules`. To refine the search, use it together with other commands, such as [`--exclude`](#exclude).

```bash
npx miho major -r
```

### `--include`

Glob pattern indicating where to search for packages. By default, Miho will search the [current working directory](https://nodejs.org/dist/latest/docs/api/process.html#processcwd) (and also subdirectories, if [`--recursive`](#recursive)).

```bash
npx miho major -r -i testdir/**
```

::: warning
If the search is not recursive, this option is ignored. Miho will only search the current directory.
:::

### `--exclude`

Glob pattern indicating where Miho should **not** look for packages.

```bash
npx miho patch -r -x testdir/**
```

### `--filter`

Package names that should be filtered. Strings in the format `/abc/` will be treated as regex.

```bash
npx miho patch -r -f my-project /onlytest/
```

### `--overrides`

Allows each package to be configured individually. Note that it is more appropriate to use a [config file](../index.md#config-file) in cases like this.

```bash
npx miho premajor -p beta -r -o.test=patch
```

### `--preid`

Prerelease identifier. Only relevant when the release type is `premajor`, `preminor` or `prepatch`.

```bash
npx miho preminor -p alpha
```
