import process from 'node:process';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import semver from 'semver';
import chalk from 'chalk';
import { Miho } from '../index';
import { loadMihoConfig } from '../config';
import { prompt } from './prompt';
import type { MihoOptions } from '../types';

async function init() {
  const l = console.log;

  const argv = await yargs(hideBin(process.argv))
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
        desc: `Glob patterns indicating where to ${chalk.bold(
          'NOT'
        )} search for packages.`,
        type: 'array',
        alias: 'x'
      },
      filter: {
        desc: 'Package names to filter.',
        type: 'array',
        alias: 'f'
      },
      overrides: {
        desc: 'Allow to configure each package individually.',
        type: 'string',
        alias: 'o'
      }
    })
    .scriptName('miho')
    .parse();

  const options: Partial<MihoOptions> = {};

  if (argv._[0]) {
    options.release = argv._[0];
  }

  if (typeof argv.preid === 'string') {
    options.preid = argv.preid;
  }

  if (typeof argv.recursive === 'boolean') {
    options.recursive = argv.recursive;
  }

  if (Array.isArray(argv.include)) {
    options.include = argv.include.map((i) => i.toString());
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

  if (argv.overrides && typeof argv.overrides === 'object') {
    options.overrides = argv.overrides;
  }

  const config = await loadMihoConfig(options);

  const miho = await Miho.init(config);
  let packages = miho.getPackages({
    filter: (pkg) => Boolean(semver.valid(pkg.version))
  });

  if (packages.length === 0) {
    l(chalk.red.bold('No valid package found.'));
    return;
  }

  packages.forEach((pkg) => {
    const name = pkg.name ? chalk.bold(pkg.name) : chalk.gray.dim('NO NAME');
    const version = chalk.blue.dim(pkg.version);
    const newVersion = pkg.newVersion
      ? chalk.green.bold(pkg.newVersion)
      : chalk.red.bold('INVALID VERSION');

    l(`[ ${chalk.bold(pkg.id)}: ${name} ]  ${version}  =>  ${newVersion}`);
  });

  packages = packages.filter((pkg) => Boolean(semver.valid(pkg.newVersion)));
  if (packages.length === 0) {
    l(chalk.red.bold('No semver compliant package.'));
    l(`Check: ${chalk.underline('https://semver.org/')}`);
    return;
  }

  if (argv.ask) {
    await prompt(miho, packages);
  } else {
    await miho.bumpAll();
    l(chalk.green.bold('Packages bumped.'));
  }
}

init().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
