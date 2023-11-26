import chalk from 'chalk';
import { promptUser } from './prompt';
import type { Miho, FileData } from 'src';

interface BumpOptions {
  miho: Miho;
  packages: FileData[];
  ask: boolean;
}

export async function bump(options: BumpOptions): Promise<number> {
  const { miho, packages, ask } = options;
  let packagesBumped: number = 0;

  if (ask) {
    packagesBumped = await promptUser(miho, packages);
  } else {
    packagesBumped = await miho.bumpAll();
    miho.l`${chalk.green.bold(`${packagesBumped} package(s) bumped.`)}`;
  }

  return packagesBumped;
}
