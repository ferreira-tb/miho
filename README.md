# Miho

Easily bump your package.json version.

## CLI

|                        Command                        | Alias | Description                                                         |
| :---------------------------------------------------: | :---- | :------------------------------------------------------------------ |
|       [`--ask`](https://tb.dev.br/miho/cli#ask)       | none  | Determines whether Miho should ask for confirmation before bumping. |
| [`--recursive`](https://tb.dev.br/miho/cli#recursive) | `-r`  | Recursively bumps all packages in the monorepo.                     |
|   [`--include`](https://tb.dev.br/miho/cli#include)   | `-i`  | Glob pattern indicating where to search for packages.               |
|   [`--exclude`](https://tb.dev.br/miho/cli#exclude)   | `-x`  | Glob patterns indicating where to **NOT** search for packages.      |
|    [`--filter`](https://tb.dev.br/miho/cli#filter)    | `-f`  | Package names to filter. May be regex.                              |
| [`--overrides`](https://tb.dev.br/miho/cli#overrides) | `-o`  | Allow to configure each package individually.                       |
|     [`--preid`](https://tb.dev.br/miho/cli#preid)     | `-p`  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.           |

## Node

```ts
import { Miho } from 'miho';

// Set up Miho and search for packages.
const options = {
  release: 'patch',
  recursive: true,
  ignore: [/test/],
  overrides: {
    'that-project': 'major'
  }
};

const miho = await new Miho().search(options);

// Get basic information on the packages found.
// This also returns an id identifying each package,
// which can eventually be used to bump them individually.
console.log(miho.getPackages());

// Bump a package by its id.
await miho.bump(package.id);

// Bump all the packages found by Miho.
await miho.bumpAll();
```

## Documentation

Read the [documentation](https://tb.dev.br/miho) for more details.

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
