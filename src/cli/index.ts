import process from 'node:process';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import semver from 'semver';
import chalk from 'chalk';
import { Miho } from '../index';
import { loadMihoConfig } from '../config';
import { prompt } from './prompt';
import { LogLevel } from '../utils';
import type { CliOptions, CliFlag } from '../types';

async function init() {
  const argv = await yargs(hideBin(process.argv))
    .scriptName('miho')
    .options({
      ask: {
        desc: 'Determines whether Miho should ask for confirmation.',
        type: 'boolean',
        default: true
      },
      preid: {
        desc: 'Prerelease identifier.',
        type: 'string',
        alias: 'p'
      },
      recursive: {
        desc: 'Recursively bumps all packages in the monorepo.',
        type: 'boolean',
        alias: 'r'
      },
      include: {
        desc: 'Glob pattern indicating where to search for packages.',
        type: 'array',
        alias: 'i'
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
      silent: {
        desc: 'Omit unimportant logs.',
        type: 'boolean'
      },
      verbose: {
        desc: 'Log additional info.',
        type: 'boolean'
      },
      overrides: {
        desc: 'Allow to configure each package individually.',
        type: 'string',
        alias: 'o'
      }
    } satisfies CliFlag)
    .parse();

  const options: Partial<CliOptions> = {};

  if (argv._[0]) {
    options.release = argv._[0];
  }

  if (Array.isArray(argv.exclude)) {
    options.exclude = argv.exclude.map((i) => i.toString());
  }

  if (Array.isArray(argv.filter)) {
    options.filter = argv.filter.map((i) => {
      const value = i.toString();
      if (/^\/.*\/$/.test(value)) {
        try {
          return new RegExp(value);
        } catch {
          return value;
        }
      }

      return value;
    });
  }

  if (Array.isArray(argv.include)) {
    options.include = argv.include.map((i) => i.toString());
  }

  if (argv.overrides && typeof argv.overrides === 'object') {
    options.overrides = argv.overrides;
  }

  if (typeof argv.preid === 'string') {
    options.preid = argv.preid;
  }

  if (typeof argv.recursive === 'boolean') {
    options.recursive = argv.recursive;
  }

  if (typeof argv.silent === 'boolean') {
    options.silent = argv.silent;
  }

  if (typeof argv.verbose === 'boolean') {
    options.verbose = argv.verbose;
  }

  const config = await loadMihoConfig(options);
  const miho = await new Miho(config).search();

  let packages = miho.getPackages({
    filter: (pkg) => Boolean(semver.valid(pkg.version))
  });

  if (packages.length === 0) {
    miho.l`${chalk.red.bold('No valid package found.')}`;
    return;
  }

  packages.forEach((pkg) => {
    const name = pkg.name ? chalk.bold(pkg.name) : chalk.gray.dim('NO NAME');
    const version = chalk.blue.dim(pkg.version);
    const newVersion = pkg.newVersion
      ? chalk.green.bold(pkg.newVersion)
      : chalk.red.bold('INVALID VERSION');

    miho.l`[ ${chalk.bold(pkg.id)}: ${name} ]  ${version}  =>  ${newVersion}`;
  });

  packages = packages.filter((pkg) => Boolean(semver.valid(pkg.newVersion)));
  if (packages.length === 0) {
    miho.l`${chalk.red.bold('No semver compliant package.')}`;
    miho.l(LogLevel.NORMAL)`Check: ${chalk.underline('https://semver.org/')}`;
    return;
  }

  if (argv.ask) {
    await prompt(miho, packages);
  } else {
    const amount = await miho.bumpAll();
    miho.l`${chalk.green.bold(`${amount} package(s) bumped.`)}`;
  }
}

init().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
