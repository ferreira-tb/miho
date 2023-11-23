# Miho

Easily bump your package.json version.

## Config file

It is recommended to use a config file whenever possible. It should be named `miho.config.ts`, but other extensions are also accepted, such as `.js`. For more details on the options available, [check the API](https://tb.dev.br/miho/api/interfaces/MihoOptions.html).

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

## CLI

|                             Command                              | Alias | Description                                                         |
| :--------------------------------------------------------------: | :---- | :------------------------------------------------------------------ |
|       [`--ask`](https://tb.dev.br/miho/usage/cli.html#ask)       | none  | Determines whether Miho should ask for confirmation before bumping. |
| [`--recursive`](https://tb.dev.br/miho/usage/cli.html#recursive) | `-r`  | Recursively bumps all packages in the monorepo.                     |
|   [`--include`](https://tb.dev.br/miho/usage/cli.html#include)   | `-i`  | Glob pattern indicating where to search for packages.               |
|   [`--exclude`](https://tb.dev.br/miho/usage/cli.html#exclude)   | `-x`  | Glob patterns indicating where to **NOT** search for packages.      |
|    [`--filter`](https://tb.dev.br/miho/usage/cli.html#filter)    | `-f`  | Package names to filter. May be regex.                              |
| [`--overrides`](https://tb.dev.br/miho/usage/cli.html#overrides) | `-o`  | Allow to configure each package individually.                       |
|     [`--preid`](https://tb.dev.br/miho/usage/cli.html#preid)     | `-p`  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.           |

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
