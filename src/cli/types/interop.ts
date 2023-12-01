import type { JobOptions } from '../../jobs';
import type { CliCommitOptions, CliOptions } from '.';

export type CliOnly = 'ask';
export type CliHasDifferentType = 'exclude' | 'include';

export type InterchangeableCliOptions = Omit<
  CliOptions,
  keyof CliCommitOptions | keyof JobOptions | CliOnly | CliHasDifferentType
>;
