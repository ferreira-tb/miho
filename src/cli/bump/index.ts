import chalk from 'chalk';
import { promptUser } from './prompt';
import type { Miho, FileData } from 'src';

/**
 * @internal
 * @ignore
 */
export interface BumpArgs {
  miho: Miho;
  packages: FileData[];
  ask: boolean;
  dryRun: boolean;
}

export async function bump(args: BumpArgs): Promise<number> {
  const { miho, ask } = args;
  let packagesBumped: number = 0;

  if (ask) {
    packagesBumped = await promptUser(args);
  } else {
    packagesBumped = await miho.bumpAll();
    miho.l`${chalk.green.bold(`${packagesBumped} package(s) bumped.`)}`;
  }

  return packagesBumped;
}
