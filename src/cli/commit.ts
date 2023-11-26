import { LogLevel } from '../utils';
import type { Miho } from '../miho';
import type { MihoOptions } from '../types';

interface CliCommitFunctionOptions {
  miho: Miho;
  config: Partial<MihoOptions>;
  packagesBumped: number;
}

export async function commit(options: CliCommitFunctionOptions) {
  const { miho, config, packagesBumped } = options;
  if (
    (typeof config.commit?.message === 'string' && packagesBumped > 0) ||
    config.commit?.all === true
  ) {
    miho.l(LogLevel.NORMAL)`Committing files...`;
    await miho.commit();
  }
}
