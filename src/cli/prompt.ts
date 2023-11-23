import chalk from 'chalk';
import prompts from 'prompts';
import type { Miho } from '../miho';
import type { PackageData } from '../files';

/** @internal */
export async function prompt(miho: Miho, packages: PackageData[]) {
  const l = console.log;

  if (packages.length === 1) {
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
        l(chalk.green.bold('Package bumped.'));
      } else {
        l(chalk.red.bold(`Could not bump package${name ? ` "${name}"` : ''}.`));
      }
    }
  } else {
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

    let amount: number = 0;
    switch (response.bumpMode) {
      case 'none':
        amount = 0;
        break;
      case 'all': {
        amount = await miho.bumpAll();
        break;
      }
      case 'some': {
        const list = response.packageList as number[];
        const results = await Promise.all(list.map(miho.bump.bind(miho)));
        amount = results.filter(Boolean).length;
        break;
      }
    }

    l(chalk.green.bold(`${amount} package(s) bumped.`));
  }
}
