import { MihoJob } from '../utils/enum';
import type { JobOptions, PartialNullish } from '../types';

export function createJobSkipChecker(options: PartialNullish<JobOptions>) {
  const { skip, only, dryRun } = options;
  const choices = Array.isArray(skip) ? skip.filter(isMihoJob) : null;
  const onlyChoice = isMihoJob(only) ? only : null;

  return function (choice: MihoJob) {
    if (dryRun === true) return true;
    if (onlyChoice && choice !== onlyChoice) return true;
    if (!choices) return false;
    return choices.includes(choice);
  };
}

export function isMihoJob(value: unknown): value is MihoJob {
  return Object.values(MihoJob).some((choice) => value === choice);
}
