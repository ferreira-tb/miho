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
    build: {
      desc: 'Build the project.',
      type: 'boolean',
      alias: 'b'
    },
    commit: {
      desc: 'Commit the modified packages.',
      type: 'string',
      alias: ['c', 'message', 'm']
    },
    dryRun: {
      desc: 'Skip all jobs.',
      type: 'boolean',
      alias: ['dry-run', 'dry'],
      default: false
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
    noVerify: {
      desc: 'Bypass pre-commit and commit-msg hooks.',
      type: 'boolean',
      alias: ['n', 'no-verify']
    },
    only: {
      desc: 'Execute only one job.',
      type: 'string',
      alias: 'l'
    },
    overrides: {
      desc: 'Allow to configure each package individually.',
      type: 'string',
      alias: 'o'
    },
    packageManager: {
      desc: 'Package manager being used.',
      type: 'string',
      alias: ['pm', 'package-manager']
    },
    preid: {
      desc: 'Prerelease identifier.',
      type: 'string'
    },
    publish: {
      desc: 'Publish the package.',
      type: 'boolean'
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
    skip: {
      desc: 'Skip one or more jobs.',
      type: 'array',
      alias: 's'
    },
    test: {
      desc: 'Run tests.',
      type: 'boolean',
      alias: 't'
    },
    verbose: {
      desc: 'Log additional info. May be useful for debugging.',
      type: 'boolean'
    }
  };

  return options;
}
