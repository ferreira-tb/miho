import type { Options, Arguments } from 'yargs';
import type { PackageOptions } from './index';
import type { CommitOptions } from './git';
import type { WithRequired } from './utils';
import type { PackageManager } from '../utils/enum';
import type { JobOptions } from './jobs';

export interface CliCommitOptions extends Omit<CommitOptions, 'message'> {
  commit: string;
}

export type CliOverrides = Record<
  string,
  PackageOptions['release'] | Partial<PackageOptions>
>;

export interface CliOptions
  extends PackageOptions,
    CliCommitOptions,
    JobOptions {
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
   * Glob pattern indicating where to search for packages.
   *
   * By default, Miho will search the current directory.
   */
  include: string[];
  /**
   * Each key represents the name of a package.
   * From here you can configure each one individually.
   */
  overrides: CliOverrides;
  /**
   * Package manager being used.
   * @default 'npm'
   */
  packageManager: PackageManager;
  /**
   * Recursively bumps all packages in the monorepo.
   * @default false
   */
  recursive: boolean;
  /**
   * Omit unimportant logs.
   * @default false
   */
  silent: boolean;
  /**
   * Log additional info. May be useful for debugging.
   * @default false
   */
  verbose: boolean;
}

export type CliFlag = Record<
  Exclude<keyof CliOptions, 'release'> | 'ask',
  WithRequired<Options, 'desc' | 'type'>
>;

export type CliArguments = Arguments<CliOptions>;
