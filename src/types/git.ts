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

export interface MihoCommitArgs extends CommitOptions {
  /** @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---dry-run */
  dryRun?: boolean;
}
