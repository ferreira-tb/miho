import process from 'node:process';
import chalk from 'chalk';
import prompts from 'prompts';
import type { PromptArgs } from './types';
import { MihoJob, logDryRun } from '../../utils';

/** @internal */
export async function promptUser(args: PromptArgs): Promise<number> {
  if (args.packages.length === 1) {
    return await promptSingle(args);
  }
  return await promptMultiple(args);
}

async function promptSingle(args: PromptArgs): Promise<number> {
  const { miho, packages, dryRun } = args;
  const name = packages[0].name;
  const response = await prompts(
    {
      name: 'confirm',
      type: 'toggle',
      message: `Bump${name ? ` ${name}` : ''}?`,
      initial: true,
      active: 'yes',
      inactive: 'no'
    },
    {
      onCancel: () => process.exit(1)
    }
  );

  if (!response.confirm) process.exit(1);

  if (dryRun) {
    logDryRun(miho, MihoJob.BUMP);
    return 0;
  }

  const result = await miho.bump(packages[0].id);
  if (result) {
    miho.l`${chalk.green.bold('Package bumped.')}`;
    return 1;
  }

  const msg = `Could not bump package${name ? ` "${name}"` : ''}.`;
  miho.l`${chalk.red.bold(msg)}`;

  return 0;
}

async function promptMultiple(options: PromptArgs): Promise<number> {
  const { miho, packages, dryRun } = options;
  const response = await prompts(
    [
      {
        name: 'bumpMode',
        type: 'select',
        message: 'Select what to bump.',
        choices: [
          { title: 'all', value: 'all' },
          { title: 'some', value: 'some' },
          { title: 'none', value: 'none' }
        ]
      },
      {
        name: 'packageList',
        type: (p) => (p === 'some' ? 'multiselect' : null),
        message: 'Select the packages to bump.',
        choices: packages.map((pkg) => {
          return {
            title: `${pkg.id}:  ${pkg.name ?? 'NO NAME'}`,
            value: pkg.id
          };
        })
      }
    ],
    {
      onCancel: () => process.exit(1)
    }
  );

  if (response.bumpMode === 'none') {
    process.exit(1);
  }

  if (dryRun) {
    logDryRun(miho, MihoJob.BUMP);
    return 0;
  }

  let packagesBumped = 0;
  switch (response.bumpMode) {
    case 'all': {
      packagesBumped = await miho.bumpAll();
      break;
    }
    case 'some': {
      const list = response.packageList as number[];
      const results = await Promise.all(list.map(miho.bump.bind(miho)));
      packagesBumped = results.filter(Boolean).length;
      break;
    }
    default:
      throw new Error('Invalid prompt mode.');
  }

  miho.l`${chalk.green.bold(`${packagesBumped} package(s) bumped.`)}`;
  return packagesBumped;
}
