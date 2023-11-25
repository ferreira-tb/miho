export interface CommitOptions {
  /**
   * @default 'chore: bump version'
   * @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt--mltmsggt
   */
  message: string;
  /**
   * @default false
   * @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---all
   */
  all: boolean;
  /**
   * @default false
   * @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---no-verify
   */
  'no-verify': boolean;
}
