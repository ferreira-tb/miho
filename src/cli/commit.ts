import { LogLevel } from '../utils';
import type { Miho } from '../miho';
import type { MihoOptions } from '../types';

interface CliCommitFunctionArgs {
  miho: Miho;
  config: Partial<MihoOptions>;
  packagesBumped: number;
  dryRun: boolean;
}

export async function commit(args: CliCommitFunctionArgs) {
  const { miho, config, packagesBumped } = args;
  if (
    (typeof config.commit?.message === 'string' && packagesBumped > 0) ||
    config.commit?.all === true
  ) {
    miho.l(LogLevel.NORMAL)`Committing files...`;
    await miho.commit({ dryRun: args.dryRun });
  }
}
