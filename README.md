# Miho

Easily bump your package.json version.

## CLI

|                        Command                        | Alias | Description                                                         |
| :---------------------------------------------------: | :---- | :------------------------------------------------------------------ |
|       [`--ask`](https://tb.dev.br/miho/cli#ask)       | none  | Determines whether Miho should ask for confirmation before bumping. |
|   [`--exclude`](https://tb.dev.br/miho/cli#exclude)   | `-x`  | Glob patterns indicating where to **NOT** search for packages.      |
|    [`--filter`](https://tb.dev.br/miho/cli#filter)    | `-f`  | Package names to filter. May be regex.                              |
|   [`--include`](https://tb.dev.br/miho/cli#include)   | `-i`  | Glob pattern indicating where to search for packages.               |
| [`--overrides`](https://tb.dev.br/miho/cli#overrides) | `-o`  | Allow to configure each package individually.                       |
|     [`--preid`](https://tb.dev.br/miho/cli#preid)     | `-p`  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.           |
| [`--recursive`](https://tb.dev.br/miho/cli#recursive) | `-r`  | Recursively bumps all packages in the monorepo.                     |
|    [`--silent`](https://tb.dev.br/miho/cli#silent)    | none  | Omit unimportant logs.                                              |
|   [`--verbose`](https://tb.dev.br/miho/cli#verbose)   | none  | Log additional info.                                                |

## Javascript API

```ts
import { Miho } from 'miho';

// Set up Miho and search for packages.
const miho = new Miho({
  release: 'patch',
  recursive: true,
  exclude: ['testdir/**'],
  filter: [/test/],
  overrides: {
    'that-project': 'major'
  }
});

await miho.search(options);

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
