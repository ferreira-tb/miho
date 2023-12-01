import type { FileData } from '../files';
import type { JobOptions } from '../jobs';
import type { CommitOptions } from '../git';
import type { InterchangeableCliOptions } from '../cli/types';

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
  jobs?: Partial<JobOptions>;
}

export interface MihoGetPackagesOptions {
  filter?: (pkg: FileData) => boolean;
}

export interface MihoCommitArgs extends CommitOptions {
  /** @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---dry-run */
  dryRun?: boolean;
}
