/**
 * @see https://git-scm.com/docs/git-commit
 */
export const enum CommitCommand {
  /** @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---all */
  ALL = '--all',
  /** @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---dry-run */
  DRY_RUN = '--dry-run',
  /** @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---messageltmsggt */
  MESSAGE = '--message',
  /** @see https://git-scm.com/docs/git-commit#Documentation/git-commit.txt---no-verify */
  NO_VERIFY = '--no-verify'
}

export const enum CommitDefaults {
  DEFAULT_MESSAGE = 'chore: bump version'
}

export const enum FileType {
  PACKAGE_JSON = 'package.json'
}

/**
 * @internal
 * @ignore
 */
export const enum LogLevel {
  /** Only displayed if `--verbose` flag is set. */
  LOW = 0,
  /** Not so important. Can be omitted if `--silent`. */
  NORMAL = 1,
  /** Important log. Should always be displayed. */
  HIGH = 2
}

export const enum MihoIgnore {
  GIT = '**/.git/**',
  NODE_MODULES = '**/node_modules/**'
}

export enum MihoJob {
  BUILD = 'build',
  BUMP = 'bump',
  COMMIT = 'commit',
  PUBLISH = 'publish',
  PUSH = 'push',
  TEST = 'test'
}

export enum PackageManager {
  NPM = 'npm',
  PNPM = 'pnpm',
  YARN = 'yarn'
}
