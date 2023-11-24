import type { FileData } from '../index';
import type { CliOptions, CliCommitOptions } from './cli';
import type { CommitOptions } from './git';
import type { MihoHooks } from './hooks';

export type * from './cli';
export type * from './git';
export type * from './hooks';
export type * from './utils';

export interface PackageOptions {
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
}

export type MihoInternalOptions = Omit<CliOptions, keyof CliCommitOptions>;

export interface MihoOptions extends MihoInternalOptions {
  /**
   * @default false
   * @see https://git-scm.com/docs/git-commit
   */
  commit?: Partial<CommitOptions>;

  /**
   * @see https://tb.dev.br/miho/hooks
   */
  hooks?: Partial<MihoHooks>;
}

export type GetPackagesOptions = {
  filter?: (pkg: FileData) => boolean;
};
