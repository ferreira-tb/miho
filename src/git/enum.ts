/**
 * @see https://git-scm.com/docs/git-commit
 */
export const enum CommitCommand {
  ALL = '--all',
  MESSAGE = '--message',
  NO_VERIFY = '--no-verify'
}

export const enum CommitDefaults {
  DEFAULT_MESSAGE = 'chore: bump version'
}
