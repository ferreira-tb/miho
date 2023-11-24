import type { MaybeArray, MaybePromise } from './utils';
import type { Miho, PackageData } from '../index';
import type { Options } from 'yargs';

export type * from './utils';

export type PackageOptions = {
  /**
   * This option will be applied to every package found by Miho.
   *
   * If this is a number, Miho will try to coerce it to a valid version.
   *
   * You can override this for individual packages in the config file.
   * @default 'patch'
   */
  release: string | number;
  /**
   * Prerelease identifier, like the `beta` in `1.0.0-beta.1`.
   * @default 'alpha'
   */
  preid: string;
};

export type CliOptions = PackageOptions & {
  /**
   * Recursively bumps all packages in the monorepo.
   * @default false
   */
  recursive: boolean;
  /**
   * Glob pattern indicating where to search for packages.
   *
   * By default, Miho will search the current directory.
   */
  include: string | string[];
  /**
   * Glob patterns indicating where to **NOT** search for packages.
   * `.git` and `node_modules` are **ALWAYS** excluded.
   */
  exclude: string[];
  /**
   * Package names to filter.
   */
  filter: (string | RegExp)[];
  /**
   * Omit unimportant logs.
   * @default false
   */
  silent: boolean;
  /**
   * Log additional info.
   * @default false
   */
  verbose: boolean;
  /**
   * Each key represents the name of a package.
   * From here you can configure each one individually.
   */
  overrides?: Record<
    string,
    PackageOptions['release'] | Partial<PackageOptions>
  >;
};

export type CliFlag = Record<
  Exclude<keyof CliOptions, 'release'> | 'ask',
  Options
>;

export interface HookCallbackParameters<T> {
  miho: Miho;
  data: T;
}

export type HookBeforeAllCallback = (
  data: HookCallbackParameters<PackageData[]>
) => MaybePromise<boolean | void>;

export type HookAfterAllCallback = (
  data: HookCallbackParameters<PackageData[]>
) => MaybePromise<void>;

export type HookBeforeEachCallback = (
  data: HookCallbackParameters<PackageData>
) => MaybePromise<boolean | void>;

export type HookAfterEachCallback = (
  data: HookCallbackParameters<PackageData>
) => MaybePromise<void>;

export type MihoHooks = {
  readonly beforeAll: MaybeArray<HookBeforeAllCallback>;
  readonly afterAll: MaybeArray<HookAfterAllCallback>;
  readonly beforeEach: MaybeArray<HookBeforeEachCallback>;
  readonly afterEach: MaybeArray<HookAfterEachCallback>;
};

export type MihoHookCallback<T extends keyof MihoHooks> = T extends 'beforeAll'
  ? HookBeforeAllCallback
  : T extends 'afterAll'
    ? HookAfterAllCallback
    : T extends 'beforeEach'
      ? HookBeforeEachCallback
      : T extends 'afterEach'
        ? HookAfterEachCallback
        : never;

export type MihoOptions = CliOptions & {
  readonly hooks?: Partial<MihoHooks>;
};

export type GetPackagesOptions = {
  filter?: (pkg: PackageData) => boolean;
};
