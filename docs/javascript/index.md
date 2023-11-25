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
  commit: {
    message: 'a commit message',
    all: true,
    push: true
  },
  overrides: {
    'main-project': 'major',
    'my-other-project': {
      release: 'preminor',
      preid: 'beta'
    }
  }
});

await miho.search(options);

// Get basic information on the packages found.
// This also returns an id identifying each package,
// which can eventually be used to bump them individually.
console.log(miho.getPackages());

// Register hooks.
miho.beforeEach(async ({ data }) => {
  await doSomethingAsync(data);
});

// Bump a package by its id.
await miho.bump(package.id);

// Bump all the packages found by Miho.
await miho.bumpAll();
```

## Options

Most of these options are already explained in the [CLI](../cli/index.md) section, so it's recommended that you take a look.

```ts
interface MihoOptions {
  commit?: {
    message: string;
    all: boolean;
    'no-verify': boolean;
  };
  exclude: string | string[];
  filter: (string | RegExp)[];
  hooks?: Partial<MihoHooks>;
  include: string | string[];
  overrides?: Record<string, MihoOptions['release'] | Partial<PackageOptions>>;
  preid: string;
  recursive: boolean;
  release: string | number;
  silent: boolean;
  verbose: boolean;
}
```

::: tip Hooks
Check the [Hooks](../hooks/index.md) section for more details on hooks.
:::

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

### commit

```ts
interface CommitOptions {
  all: boolean;
  message: string;
  'no-verify': boolean;
  push: boolean;
}

interface Miho {
  commit(options?: Partial<CommitOptions>): Promise<void>;
}
```

Commit the modified packages.

This will throw an error if called while no package has been modified.

### getPackageByName

```ts
interface Miho {
  getPackageByName(): FileData | null;
}
```

Find a package by its name among the ones previously found by Miho.

### getPackages

```ts
type MihoGetPackagesOptions = {
  filter?: (pkg: FileData) => boolean;
};

interface Miho {
  getPackages(options?: MihoGetPackagesOptions): FileData[];
}
```

Returns information on the packages found by Miho.

The `FileData` objects are just a snapshot of the packages at the time they were found. Modifying any property will have no effect on them.

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
  beforeEach: ({ data }) => data.id === 1,
  afterEach: ({ data }) => console.log(data),
  beforeAll: ({ data }) => data.every(({ id }) => id > 1),
  afterAll: ({ data }) => data.forEach((pkg) => console.log(pkg))
});
```

### search

```ts
interface Miho {
  search(options?: Partial<MihoOptions>): Promise<Miho>;
}
```

Search for all packages that meet the requirements. If `options` is defined, it will override those previously given to the constructor.
