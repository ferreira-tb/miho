import chalk from 'chalk';
import type { Miho } from '../miho';
import type { MihoJob } from './enum';

export * from './enum';

export function isNotBlank(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0;
}

/**
 * @internal
 * @ignore
 */
export function isTemplateArray(value: unknown): value is TemplateStringsArray {
  return Array.isArray(value);
}

/**
 * @internal
 * @ignore
 */
export function logDryRun(miho: Miho, job: MihoJob) {
  miho.l`${chalk.yellow('[DRY RUN]')} ${job} skipped.`;
}
