import type { Options as ExecaOptions } from 'execa';
import type { Miho } from '../miho';
import type { Nullish } from '../utils';
import type { MihoPackage } from '../files';

interface BaseArgs {
  dryRun?: Nullish<boolean>;
  execaOptions?: ExecaOptions;
  miho: Miho;
}

export interface CommitArgs extends BaseArgs {
  packages: MihoPackage[];
}

export interface PushCommitArgs extends BaseArgs {
  /** @see https://git-scm.com/docs/git-push#Documentation/git-push.txt---dry-run */
  dryRun?: Nullish<boolean>;
}

export type HandleExceptionArgs = Required<Omit<BaseArgs, 'execaOptions'>>;

export interface CommitOptions {
  /**
   * @default false
   * @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---all
   */
  all: boolean;
  /**
   * @default 'chore: bump version'
   * @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt--mltmsggt
   */
  message: string;
  /**
   * @default false
   * @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---no-verify
   */
  noVerify: boolean;
  /**
   * Push the commit.
   * @default false
   * @see https://git-scm.com/docs/git-push
   */
  push: boolean;
}
