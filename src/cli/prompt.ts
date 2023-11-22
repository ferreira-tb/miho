import chalk from 'chalk';
import prompts from 'prompts';
import type { Miho } from '../miho';
import type { PackageData } from '../types';

/** @internal */
export async function prompt(miho: Miho, packages: PackageData[]) {
  const l = console.log;

  if (packages.length === 1) {
    const name = packages[0].name;
    const result = await prompts({
      name: 'confirm',
      type: 'toggle',
      message: `Bump${name ? ` ${name}` : ''}?`,
      initial: true,
      active: 'yes',
      inactive: 'no'
    });

    if (result.confirm === true) {
      await miho.bump(packages[0].id);
      l(chalk.green.bold('Package bumped.'));
    }
  } else {
    const result = await prompts([
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

    switch (result.bumpMode) {
      case 'none':
        return;
      case 'all': {
        await miho.bumpAll();
        l(chalk.green.bold('Packages bumped.'));
        break;
      }
      case 'some': {
        const list = result.packageList as number[];
        await Promise.all(list.map(miho.bump.bind(miho)));
        l(chalk.green.bold(`Package${list.length > 1 ? 's' : ''} bumped.`));
      }
    }
  }
}
