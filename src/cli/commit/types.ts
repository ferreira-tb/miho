import type { Miho, MihoOptions } from '../../miho';

export interface CliCommitFunctionArgs {
  config: Partial<MihoOptions>;
  dryRun: boolean;
  miho: Miho;
  packagesBumped: number;
}
