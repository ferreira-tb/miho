import chalk from 'chalk';
import prompts from 'prompts';
import type { Miho } from '../../miho';
import type { FileData } from '../../files';

/** @internal */
export async function promptUser(
  miho: Miho,
  packages: FileData[]
): Promise<number> {
  if (packages.length === 1) {
    return await promptSingle(miho, packages);
  } else {
    return await promptMultiple(miho, packages);
  }
}

async function promptSingle(miho: Miho, packages: FileData[]): Promise<number> {
  const name = packages[0].name;
  const response = await prompts({
    name: 'confirm',
    type: 'toggle',
    message: `Bump${name ? ` ${name}` : ''}?`,
    initial: true,
    active: 'yes',
    inactive: 'no'
  });

  if (response.confirm === true) {
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

async function promptMultiple(
  miho: Miho,
  packages: FileData[]
): Promise<number> {
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
