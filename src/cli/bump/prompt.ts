import chalk from 'chalk';
import prompts from 'prompts';
import { MihoJob, skipAsDryRun } from '../../utils';
import type { BumpArgs } from './index';

type PromptArgs = Omit<BumpArgs, 'ask'>;

/** @internal */
export async function promptUser(args: PromptArgs): Promise<number> {
  if (args.packages.length === 1) {
    return await promptSingle(args);
  } else {
    return await promptMultiple(args);
  }
}

async function promptSingle(args: PromptArgs): Promise<number> {
  const { miho, packages, dryRun } = args;
  const name = packages[0].name;
  const response = await prompts({
    name: 'confirm',
    type: 'toggle',
    message: `Bump${name ? ` ${name}` : ''}?`,
    initial: true,
    active: 'yes',
    inactive: 'no'
  });

  if (dryRun) {
    skipAsDryRun(miho, MihoJob.BUMP);
    return 0;
  } else if (response.confirm === true) {
    const result = await miho.bump(packages[0].id);
    if (result) {
      miho.l`${chalk.green.bold('Package bumped.')}`;
      return 1;
    } else {
      const msg = `Could not bump package${name ? ` "${name}"` : ''}.`;
      miho.l`${chalk.red.bold(msg)}`;
    }
  }

  return 0;
}

async function promptMultiple(options: PromptArgs): Promise<number> {
  const { miho, packages, dryRun } = options;
  const response = await prompts([
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
  ]);

  if (dryRun) {
    skipAsDryRun(miho, MihoJob.BUMP);
    return 0;
  }

  let packagesBumped: number = 0;
  switch (response.bumpMode) {
    case 'none':
      break;
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
