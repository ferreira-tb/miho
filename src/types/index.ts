import type { FileData } from '../index';
import type { JobOptions } from './jobs';
import type { MihoHooks } from './hooks';
import type { CommitOptions } from './git';
import type { CliCommitOptions, CliOptions } from './cli';

export type * from './cli';
export type * from './git';
export type * from './hooks';
export type * from './jobs';
export type * from './package';
export type * from './utils';

export type CliOnly = 'ask';
export type CliHasDifferentType = 'exclude' | 'include';

export type InterchangeableCliOptions = Omit<
  CliOptions,
  keyof CliCommitOptions | keyof JobOptions | CliOnly | CliHasDifferentType
>;

export interface MihoInternalOptions extends InterchangeableCliOptions {
  exclude: string | string[];
  include?: string | string[];
}

export interface MihoOptions extends MihoInternalOptions {
  /**
   * @default false
   * @see https://git-scm.com/docs/git-commit
   */
  commit?: Partial<CommitOptions>;

  /**
   * @see https://tb.dev.br/miho/hooks
   * @deprecated
   */
  hooks?: Partial<MihoHooks>;

  jobs?: Partial<JobOptions>;
}

export interface MihoGetPackagesOptions {
  filter?: (pkg: FileData) => boolean;
}
