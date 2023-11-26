import type { FileData } from '../index';
import type { CliOptions, CliCommitOptions } from './cli';
import type { CommitOptions } from './git';
import type { MihoHooks } from './hooks';

export type * from './cli';
export type * from './git';
export type * from './hooks';
export type * from './package';
export type * from './utils';

export type CliOnly = 'ask' | 'dryRun' | 'only' | 'skip';
export type CliHasDifferentType = 'exclude' | 'include';

export type InterchangeableCliOptions = Omit<
  CliOptions,
  keyof CliCommitOptions | CliOnly | CliHasDifferentType
>;

export interface MihoInternalOptions extends InterchangeableCliOptions {
  exclude: string | string[];
  include: string | string[];
}

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

  build?: boolean;
  publish?: boolean;
  test?: boolean;
}

export type MihoGetPackagesOptions = {
  filter?: (pkg: FileData) => boolean;
};
