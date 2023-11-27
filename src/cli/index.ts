import process from 'node:process';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';
import semver from 'semver';
import chalk from 'chalk';
import { Miho } from '../index';
import { loadConfig } from '../config';
import { bump } from './bump';
import { commit } from './commit';
import { normalize } from './normalize';
import { LogLevel, MihoJob, logDryRun } from '../utils';
import { createJobSkipChecker } from '../jobs';
import { createOptions } from './options';
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
  let packagesBumped: number = 0;

  const dryRun = Boolean(config.jobs?.dryRun);
  const shouldSkipJob = createJobSkipChecker({
    skip: config.jobs?.skip,
    only: config.jobs?.only
  });

  if (!shouldSkipJob(MihoJob.BUMP)) {
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

    const ask = Boolean(argv.ask);
    packagesBumped = await bump({
      miho,
      packages,
      ask,
      dryRun
    });
  }

  if (!shouldSkipJob(MihoJob.BUILD) && config.jobs?.build) {
    if (dryRun) {
      logDryRun(miho, MihoJob.BUILD);
    } else {
      await miho.build();
    }
  }

  if (!shouldSkipJob(MihoJob.TEST) && config.jobs?.test) {
    if (dryRun) {
      logDryRun(miho, MihoJob.TEST);
    } else {
      await miho.test();
    }
  }

  if (!shouldSkipJob(MihoJob.COMMIT)) {
    await commit({
      miho,
      config,
      packagesBumped,
      dryRun
    });
  }

  if (!shouldSkipJob(MihoJob.PUBLISH) && config.jobs?.publish) {
    if (dryRun) {
      logDryRun(miho, MihoJob.PUBLISH);
    } else {
      await miho.publish();
    }
  }
}

main().catch((err: unknown) => {
  console.error(err);
  process.exit(1);
});
