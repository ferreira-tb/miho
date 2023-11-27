import { MihoJob } from '../utils/enum';
import type { JobOptions, PartialNullish } from '../types';

type JobSkipCheckerArgs = Omit<PartialNullish<JobOptions>, 'dryRun'>;

export function createJobSkipChecker(args: JobSkipCheckerArgs) {
  const { skip, only } = args;
  const choices = Array.isArray(skip) ? skip.filter(isMihoJob) : null;
  const onlyChoice = isMihoJob(only) ? only : null;

  return function (choice: MihoJob) {
    if (onlyChoice && choice !== onlyChoice) return true;
    if (!choices) return false;
    return choices.includes(choice);
  };
}

export function isMihoJob(value: unknown): value is MihoJob {
  return Object.values(MihoJob).some((choice) => value === choice);
}
