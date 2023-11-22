export interface PackageOptions {
  /**
   * This option will be applied to every package found by Miho.
   *
   * If this is a number, Miho will try to coerce it to a valid version.
   *
   * You can override the release type for individual packages in the `miho.config.ts` file.
   * @default 'patch'
   */
  release: string | number;
  /**
   * Prerelease identifier, like the `beta` in `1.0.0-beta.1`.
   * @default 'alpha'
   */
  preid: string;
}

export interface MihoOptions extends PackageOptions {
  /**
   * Recursively bumps all packages in the monorepo.
   * @default false
   */
  recursive: boolean;
  /**
   * Glob pattern indicating where to search for packages.
   *
   * By default, Miho will search the current directory (and also subdirectories, if `--recursive`).
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
   * Each key represents the name of a package.
   * From here you can configure each one individually.
   */
  overrides?: Record<string, string | number | Partial<PackageOptions>>;
}

export type PackageData = {
  readonly id: number;
  readonly name: string | null;
  readonly version: string;
  readonly newVersion: string | null;
};

export type GetPackagesOptions = {
  filter?: (pkg: PackageData) => boolean;
};
