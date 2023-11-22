# Miho

Easily bump your package.json version.

```
npm i -D miho
```

## CLI Usage

|    Command    | Alias | Description                                                         |
| :-----------: | :---- | :------------------------------------------------------------------ |
|    `--ask`    | none  | Determines whether Miho should ask for confirmation before bumping. |
|   `--preid`   | `-p`  | Prerelease identifier, like the `beta` in `1.0.0-beta.1`.           |
| `--recursive` | `-r`  | Recursively bumps all packages in the monorepo.                     |
|  `--ignore`   | none  | Package names to ignore. May be regex.                              |
|  `--exclude`  | `-x`  | Glob patterns indicating where to not search for packages.          |
| `--overrides` | `-o`  | Allow to configure each package individually.                       |

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
