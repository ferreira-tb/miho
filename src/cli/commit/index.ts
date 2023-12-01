import { LogLevel } from '../../utils';
import type { CliCommitFunctionArgs } from './types';

export async function commit(args: CliCommitFunctionArgs) {
  const { miho, config, packagesBumped } = args;
  if (
    (typeof config.commit?.message === 'string' && packagesBumped > 0) ||
    config.commit?.all
  ) {
    miho.l(LogLevel.NORMAL)`Committing files...`;
    await miho.commit({ dryRun: args.dryRun });
    miho.l(LogLevel.NORMAL)`Files committed.`;
  }
}
