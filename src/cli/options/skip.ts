export enum SkipChoice {
  BUILD = 'build',
  BUMP = 'bump',
  COMMIT = 'commit',
  PUBLISH = 'publish',
  TEST = 'test'
}

interface CreateSkipCheckerOptions {
  skip: unknown;
  only: unknown;
  dryRun: unknown;
}

export function createSkipChecker(options: CreateSkipCheckerOptions) {
  const { skip, only, dryRun } = options;
  const choices = Array.isArray(skip) ? skip.filter(isSkipChoice) : null;
  const onlyChoice = isSkipChoice(only) ? only : null;

  return function (choice: SkipChoice) {
    if (dryRun === true) return true;
    if (onlyChoice && choice !== onlyChoice) return true;
    if (!choices) return false;
    return choices.includes(choice);
  };
}

function isSkipChoice(value: unknown): value is SkipChoice {
  return Object.values(SkipChoice).some((choice) => value === choice);
}
