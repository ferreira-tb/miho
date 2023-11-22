# Miho

Easily bump your package.json version.

## Config file

It is recommended to use a configuration file to better manage Miho. The file should be named `miho.config.ts`, but other extensions are also accepted, such as `.js`. For more details on the options available, [visit the API](https://tb.dev.br/miho/interfaces/MihoOptions.html).

```ts
import { defineConfig } from 'miho';

export default defineConfig({
  release: 'patch',
  recursive: false,
  overrides: {
    'project-1': 'minor'
  }
});
```

## CLI Usage

|    Command    | Alias | Description                                                         |
| :-----------: | :---- | :------------------------------------------------------------------ |
|    `--ask`    | none  | Determines whether Miho should ask for confirmation before bumping. |
| `--recursive` | `-r`  | Recursively bumps all packages in the monorepo.                     |
|  `--ignore`   | none  | Package names to ignore. May be regex.                              |
|  `--exclude`  | `-x`  | Glob patterns indicating where to not search for packages.          |
| `--overrides` | `-o`  | Allow to configure each package individually.                       |
|   `--preid`   | `-p`  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.           |

The first positional argument will always be taken as the desired release type. The value of this argument can be an arbitrary string containing a valid version, an integer or one of the constants described below:

- major
- premajor
- minor
- preminor
- patch
- prepatch

If the value of the argument is a version, Miho will bump the packages to that version. On the other hand, if the value is a number, it will perform a major bump to the version to which the number corresponds. Finally, using constants such as `major` and `patch` does what one would expect.

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

#### `--ask`

After getting the packages and being ready to bump them, Miho, by default, checks that you agree with the changes. When multiple packages are being changed at the same time, Miho also allows you to specify which ones to change.

You can change this behavior using the `--no-ask` command. That way, Miho won't ask for your confirmation and will execute the changes immediately.

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

Allows each package to be configured individually. Note that it is more appropriate to use a configuration file in cases like this.

```bash
npx miho premajor -p beta -r -o.test=patch
```

#### `--preid`

Prerelease identifier. Only considered when the release type is `premajor`, `preminor` or `prepatch`.

```bash
npx miho preminor -p alpha
```
