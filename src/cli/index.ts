import process from 'node:process';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import semver from 'semver';
import chalk from 'chalk';
import { Miho } from '../index';
import { loadConfig } from '../config';
import { bump } from './bump';
import { normalize } from './normalize';
import { LogLevel } from '../utils';
import { createOptions, createSkipChecker, SkipChoices } from './options';
import type { CliArguments } from '../types';

async function main() {
  const argv = await yargs(hideBin(process.argv))
    .scriptName('miho')
    .alias('h', 'help')
    .alias('v', 'version')
    .options(createOptions())
    .parse();

  const options = await normalize(argv as unknown as CliArguments);
  const config = await loadConfig(options);
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

  const skip = Array.isArray(argv.skip) ? (argv.skip as string[]) : null;
  const shouldSkip = createSkipChecker(skip, argv.dryRun);

  let packagesBumped: number = 0;

  if (!shouldSkip(SkipChoices.BUMP)) {
    const ask = Boolean(argv.ask);
    packagesBumped = await bump({ miho, packages, ask });
  } else {
    miho.l(LogLevel.NORMAL)`${chalk.yellow('[SKIP]')} ${SkipChoices.BUMP}`;
  }

  if (!shouldSkip(SkipChoices.COMMIT)) {
    if (
      (typeof config.commit?.message === 'string' && packagesBumped > 0) ||
      config.commit?.all === true
    ) {
      miho.l(LogLevel.NORMAL)`Committing files...`;
      await miho.commit();
    }
  } else {
    miho.l(LogLevel.NORMAL)`${chalk.yellow('[SKIP]')} ${SkipChoices.COMMIT}`;
  }
}

main().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
