# Miho

Easily bump your package.json version.

## Config file

It is recommended to use a config file. It should be named `miho.config.ts`, but other extensions are also accepted, such as `.js`. For more details on the options available, [visit the API](https://tb.dev.br/miho/api/interfaces/MihoOptions.html).

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

```bash
npx miho prepatch -r -p beta -o.test=major
```

## Node usage

```ts
import { Miho } from 'miho';

// Set up Miho and search for packages.
const miho = await Miho.init({
  release: 'patch',
  recursive: true,
  ignore: [/test/],
  overrides: {
    'that-project': 'major'
  }
});

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
