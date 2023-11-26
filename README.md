# Miho

Easily bump your package.json version.

## CLI

|                              Command                              | Alias   | Description                                                    |
| :---------------------------------------------------------------: | :------ | :------------------------------------------------------------- |
|             [`--all`](https://tb.dev.br/miho/cli#all)             | `-a`    | Commit all modified files, not only the packages.              |
|             [`--ask`](https://tb.dev.br/miho/cli#ask)             | none    | Whether Miho should ask for confirmation before bumping.       |
|          [`--commit`](https://tb.dev.br/miho/cli#commit)          | `-c`    | Commit the modified packages.                                  |
|         [`--dry-run`](https://tb.dev.br/miho/cli#dry-run)         | `--dry` | Skip all steps.                                                |
|         [`--exclude`](https://tb.dev.br/miho/cli#exclude)         | `-x`    | Glob patterns indicating where to **NOT** search for packages. |
|          [`--filter`](https://tb.dev.br/miho/cli#filter)          | `-f`    | Package names to filter. May be regex.                         |
|            [`--help`](https://tb.dev.br/miho/cli#help)            | `-h`    | Show usage information.                                        |
|         [`--include`](https://tb.dev.br/miho/cli#include)         | `-i`    | Glob patterns indicating where to search for packages.         |
|       [`--no-verify`](https://tb.dev.br/miho/cli#no-verify)       | `-n`    | Bypass `pre-commit` and `commit-msg` hooks.                    |
|            [`--only`](https://tb.dev.br/miho/cli#only)            | none    | Execute only one step.                                         |
|       [`--overrides`](https://tb.dev.br/miho/cli#overrides)       | `-o`    | Allow to configure each package individually.                  |
| [`--package-manager`](https://tb.dev.br/miho/cli#package-manager) | `--pm`  | Package manager being used.                                    |
|           [`--preid`](https://tb.dev.br/miho/cli#preid)           | none    | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.      |
|            [`--push`](https://tb.dev.br/miho/cli#push)            | `-p`    | Push the commit.                                               |
|       [`--recursive`](https://tb.dev.br/miho/cli#recursive)       | `-r`    | Recursively bumps all packages in the monorepo.                |
|          [`--silent`](https://tb.dev.br/miho/cli#silent)          | none    | Omit unimportant logs.                                         |
|            [`--skip`](https://tb.dev.br/miho/cli#skip)            | `-s`    | Skip one or more steps.                                        |
|         [`--verbose`](https://tb.dev.br/miho/cli#verbose)         | none    | Log additional info. May be useful for debugging.              |
|         [`--version`](https://tb.dev.br/miho/cli#version)         | `-v`    | Show current version.                                          |

## Javascript API

```ts
import { Miho } from 'miho';

// Set up Miho and search for packages.
const miho = new Miho({
  release: 'patch',
  recursive: true,
  exclude: ['foo/**'],
  filter: [/bar/],
  commit: {
    message: 'a commit message',
    all: true,
    push: true
  },
  overrides: {
    baz: 'major',
    qux: {
      release: 'preminor',
      preid: 'beta'
    }
  }
});

await miho.search();

// Get basic information on the packages found.
// This also returns an id identifying each package,
// which can eventually be used to bump them individually.
console.log(miho.getPackages());

// Register hooks.
miho.on('beforeEach', async (event) => {
  const { miho, data } = event;
  const result = await doSomethingAsync(miho, data);
  if (!result) event.preventDefault();
});

// Bump a package by its id.
await miho.bump(id);

// Bump all the packages found by Miho.
await miho.bumpAll();
```

## Documentation

Read the [documentation](https://tb.dev.br/miho) for more details.

## License

[MIT](https://github.com/ferreira-tb/miho/blob/main/LICENSE)
