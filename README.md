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
