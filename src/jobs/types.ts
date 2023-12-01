import type { Miho } from '../miho';
import type { MaybePromise } from '../utils/types';
import type { MihoJob, PartialNullish } from '../utils';

export type JobSkipCheckerArgs = Omit<PartialNullish<JobOptions>, 'dryRun'>;

export interface JobCallbackParams {
  cwd: string;
  miho: Miho;
  name: MihoJob;
}

export interface JobFunctionOptions {
  /**
   * @default process.cwd()
   */
  cwd?: string;
}

export interface JobFunction {
  /**
   * After the `bump` job, run the `build` script defined in the root `package.json`.
   * @default false
   */
  build: boolean | ((job: JobCallbackParams) => MaybePromise<void>);
  /**
   * After the `commit` job, execute the `publish` command (e.g. `npm publish`).
   * @default false
   */
  publish: boolean | ((job: JobCallbackParams) => MaybePromise<void>);
  /**
   * After the `build` job, run the `test` script defined in the root `package.json`.
   * @default false
   */
  test: boolean | ((job: JobCallbackParams) => MaybePromise<void>);
}

export interface JobOptions extends JobFunction {
  /**
   * Skip all jobs.
   * @default false
   */
  dryRun: boolean;
  /**
   * Execute only one job.
   *
   * Possible value is one of those used for {@link CliOptions.skip}.
   */
  only: string;

  /**
   * Skip one or more jobs.
   *
   * Possible values are `build`, `bump`, `commit`, `publish` and `test`.
   */
  skip: string[];
}
