import type { CliFlag } from '../types';

export function createOptions() {
  const options: CliFlag = {
    all: {
      desc: 'Commit all modififed files, not only the packages.',
      type: 'boolean',
      alias: 'a'
    },
    ask: {
      desc: 'Determines whether Miho should ask for confirmation.',
      type: 'boolean',
      default: true
    },
    commit: {
      desc: 'Commit the modified packages.',
      type: 'string',
      alias: 'c'
    },
    exclude: {
      desc: 'Glob patterns indicating where to NOT search for packages.',
      type: 'array',
      alias: 'x'
    },
    filter: {
      desc: 'Package names to filter.',
      type: 'array',
      alias: 'f'
    },
    include: {
      desc: 'Glob patterns indicating where to search for packages.',
      type: 'array',
      alias: 'i'
    },
    'no-verify': {
      desc: 'Bypass pre-commit and commit-msg hooks.',
      type: 'boolean',
      alias: 'n'
    },
    overrides: {
      desc: 'Allow to configure each package individually.',
      type: 'string',
      alias: 'o'
    },
    preid: {
      desc: 'Prerelease identifier.',
      type: 'string'
    },
    push: {
      desc: 'Push the commit.',
      type: 'boolean',
      alias: 'p'
    },
    recursive: {
      desc: 'Recursively bumps all packages in the monorepo.',
      type: 'boolean',
      alias: 'r'
    },
    silent: {
      desc: 'Omit unimportant logs.',
      type: 'boolean'
    },
    verbose: {
      desc: 'Log additional info. May be useful for debugging.',
      type: 'boolean'
    }
  };

  return options;
}
