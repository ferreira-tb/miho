import type { Miho } from '../../miho';
import type { FileData } from '../../files';

/** @internal */
export interface BumpArgs {
  ask: boolean;
  dryRun: boolean;
  miho: Miho;
  packages: FileData[];
}

/** @internal */
export type PromptArgs = Omit<BumpArgs, 'ask'>;
