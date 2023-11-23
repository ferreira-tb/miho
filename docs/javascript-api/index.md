---
outline: [2, 3]
---

# Javascript API

It's also possible to use Miho programmatically.

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

## Miho

```ts
interface MihoConstructor {
  new (options?: Partial<MihoOptions>): Miho;
}
```

### Options

```ts
interface PackageOptions {
  /**
   * This option will be applied to every package found by Miho.
   * If this is a number, Miho will try to coerce it to a valid version.
   * You can override this for individual packages in the config file.
   * @default 'patch'
   */
  release: string | number;
  /**
   * Prerelease identifier, like the `beta` in `1.0.0-beta.1`.
   * @default 'alpha'
   */
  preid: string;
}

interface CliOptions extends PackageOptions {
  /**
   * Recursively bumps all packages in the monorepo.
   * @default false
   */
  recursive: boolean;
  /**
   * Glob pattern indicating where to search for packages.
   * By default, Miho will search the current directory.
   */
  include: string | string[];
  /**
   * Glob patterns indicating where to NOT search for packages.
   * `.git` and `node_modules` are ALWAYS excluded.
   */
  exclude: string[];
  /**
   * Package names to filter.
   */
  filter: (string | RegExp)[];
  /**
   * Each key represents the name of a package.
   * From here you can configure each one individually.
   */
  overrides?: Record<
    string,
    PackageOptions['release'] | Partial<PackageOptions>
  >;
}

interface MihoOptions extends CliOptions {
  readonly hooks?: Partial<MihoHooks>;
}
```

Check the [Hooks](../hooks/index.md) section for more details on hooks.

## Methods

### bump

```ts
interface Miho {
  bump(id: number): Promise<boolean>;
}
```

Bumps a single package. You can get the id of the packages using the [`getPackages()`](#getpackages) method.

Returns a boolean indicating whether the package was successfully bumped or not.

```ts
const pkgs = miho.getPackages();
await miho.bump(pkgs[0].id);
```

### bumpAll

```ts
interface Miho {
  bumpAll(): Promise<number>;
}
```

Bumps all packages found by Miho.

Returns the amount of packages successfully bumped.

### getPackages

```ts
interface Miho {
  getPackages(): PackageData[];
}
```

Returns information on the packages found by Miho.

The objects returned by this method are just a snapshot of the state of the packages at the time they were found. Modifying any property will have no effect on the packages.

### resolveHooks

```ts
interface Miho {
  resolveHooks(hooks: Partial<MihoHooks>): Miho;
}
```

Register multiple hooks simultaneously.

Read the [hooks](../hooks/index.md#hooks) section for more details.

```ts
miho.resolveHooks({
  beforeEach: (data) => data.id === 1,
  afterEach: (data) => console.log(data),
  beforeAll: (data) => data.every(({ id }) => id > 1),
  afterAll: (data) => data.forEach((pkg) => console.log(pkg))
});
```

### search

```ts
interface Miho {
  search(options?: Partial<MihoOptions>): Promise<Miho>;
}
```

Search for all packages that meet the requirements. If the `options` parameter is defined, it will override those previously given to the constructor.
