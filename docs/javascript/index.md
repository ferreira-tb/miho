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
      release: 'prerelease',
      preid: 'beta'
    }
  }
});

await miho.search();

// Get basic information on the packages found.
// This also returns an id identifying each package,
// which can eventually be used to bump them individually.
console.log(miho.getPackages());

// Bump a package by its id.
await miho.bump(id);

// Bump all the packages found by Miho.
await miho.bumpAll();
```

## Options

Most of these options are already explained in the [CLI](../cli/index.md) section, so it's recommended that you take a look there.

```ts
interface PackageOptions {
  preid?: string;
  release?: string | number;
}

interface MihoOptions extends PackageOptions {
  commit?: {
    message?: string;
    all?: boolean;
    noVerify?: boolean;
  };
  exclude?: string | string[];
  filter?: (string | RegExp)[];
  include?: string | string[];
  jobs?: {
    build?: boolean | ((job: JobCallbackParams) => MaybePromise<void>);
    dryRun?: boolean;
    only?: string;
    publish?: boolean | ((job: JobCallbackParams) => MaybePromise<void>);
    skip?: string[];
    test?: boolean | ((job: JobCallbackParams) => MaybePromise<void>);
  };
  overrides?: Record<
    string,
    PackageOptions['release'] | Partial<PackageOptions>
  >;
  packageManager?: 'npm' | 'pnpm' | 'yarn';
  recursive?: boolean;
  silent?: boolean;
  verbose?: boolean;
}
```

## Methods

### build

```ts
interface JobFunctionOptions {
  cwd?: string;
}

interface Miho {
  build(options?: JobFunctionOptions): Promise<void>;
}
```

Builds the project. See [`--build`](../cli/index.md#build).

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
  noVerify: boolean;
  push: boolean;
}

interface Miho {
  commit(options?: Partial<CommitOptions>): Promise<void>;
}
```

Commit the modified packages.

### getPackageByName

```ts
interface Miho {
  getPackageByName(packageName: string | RegExp): FileData | null;
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

The `FileData` objects are just a snapshot of the packages at the time. Modifying any property will have no effect on them.

### publish

```ts
interface Miho {
  publish(options?: JobFunctionOptions): Promise<void>;
}
```

Publish the package. See [`--publish`](../cli/index.md#publish).

### search

```ts
interface Miho {
  search(options?: Partial<MihoOptions>): Promise<Miho>;
}
```

Search for all packages that meet the requirements. If `options` is defined, it will override those previously given to the constructor.

### setJob

```ts
interface Miho {
  setJob<T extends keyof JobFunction>(job: T, value: JobFunction[T]): void;
}
```

Set the value for a job.

```ts
miho.setJob('build', async () => {
  await buildMyProject();
});

await miho.build();
```

### test

```ts
interface Miho {
  test(options?: JobFunctionOptions): Promise<void>;
}
```

Run tests. See [`--test`](../cli/index.md#test).

## Functions

These are top level functions exported by Miho.

### defineConfig

```ts
import { defineConfig } from 'miho';

export default defineConfig({
  release: 'patch',
  recursive: false,
  commit: {
    message: 'a commit message',
    all: true,
    push: true
  }
});
```

Read [config file](../index.md#config-file) for details.

### detectPackageManager

<<< ../../src/utils/package-manager.ts#DetectPackageManagerOptions

```ts
declare function detectPackageManager(
  options?: DetectPackageManagerOptions
): Promise<PackageManager>;
```

Detects the package manager being used. Read [`--package-manager`](../cli/index.md#package-manager) for more details.
