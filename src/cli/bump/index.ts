import chalk from 'chalk';
import type { FileData, Miho } from 'src';
import { promptUser } from './prompt';

/**
 * @internal
 * @ignore
 */
export interface BumpArgs {
  ask: boolean;
  dryRun: boolean;
  miho: Miho;
  packages: FileData[];
}

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
