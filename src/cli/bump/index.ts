import chalk from 'chalk';
import { promptUser } from './prompt';
import type { BumpArgs } from './types';

/** @internal */
export async function bump(args: BumpArgs): Promise<number> {
  const { miho, ask } = args;
  let packagesBumped = 0;

  if (ask) {
    packagesBumped = await promptUser(args);
  } else {
    packagesBumped = await miho.bumpAll();
    miho.l`${chalk.green.bold(`${packagesBumped} package(s) bumped.`)}`;
  }

  return packagesBumped;
}
