import { LogLevel } from '../utils';
import type { Miho } from '../miho';
import type { MihoOptions } from '../types';

interface CliCommitFunctionArgs {
  config: Partial<MihoOptions>;
  dryRun: boolean;
  miho: Miho;
  packagesBumped: number;
}

export async function commit(args: CliCommitFunctionArgs) {
  const { miho, config, packagesBumped } = args;
  if (
    (typeof config.commit?.message === 'string' && packagesBumped > 0) ||
    config.commit?.all
  ) {
    miho.l(LogLevel.NORMAL)`Committing files...`;
    await miho.commit({ dryRun: args.dryRun });
  }
}
