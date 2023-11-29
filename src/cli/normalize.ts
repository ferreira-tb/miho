import {
  detectPackageManager,
  isPackageManager
} from '../utils/package-manager';
import type {
  CliArguments,
  CommitOptions,
  JobOptions,
  MihoOptions,
  PickByValue
} from '../types';

export async function normalize(
  argv: CliArguments
): Promise<Partial<MihoOptions>> {
  const options: Partial<MihoOptions> = {};
  options.commit = normalizeCommit(argv);
  options.jobs = normalizeJobs(argv);

  if (argv._[0]) {
    options.release = argv._[0];
  }

  const normalizeArgvBoolean = createBooleanNormalizer<MihoOptions>();
  const normalizeArgvString = createStringNormalizer<MihoOptions>();

  normalizeArgvBoolean(options, 'recursive', argv.recursive);
  normalizeArgvBoolean(options, 'silent', argv.silent);
  normalizeArgvBoolean(options, 'verbose', argv.verbose);

  normalizeArgvString(options, 'preid', argv.preid);

  if (!isPackageManager(argv.packageManager)) {
    options.packageManager = await detectPackageManager();
  } else {
    options.packageManager = argv.packageManager;
  }

  if (Array.isArray(argv.exclude)) {
    options.exclude = argv.exclude.map(toString);
  }

  if (Array.isArray(argv.filter)) {
    options.filter = argv.filter.map((i) => {
      const value = toString(i);
      if (/^\/.*\/$/.test(value)) {
        try {
          const regex = value.replace(/^\/|\/$/g, '');
          return new RegExp(regex);
        } catch {
          return value;
        }
      }

      return value;
    });
  }

  if (Array.isArray(argv.include)) {
    options.include = argv.include.map(toString);
  }

  if (argv.overrides && typeof argv.overrides === 'object') {
    options.overrides = argv.overrides;
  }

  return options;
}

function normalizeCommit(argv: CliArguments): Partial<CommitOptions> {
  const normalizeCommitBoolean = createBooleanNormalizer<CommitOptions>();
  const normalizeCommitString = createStringNormalizer<CommitOptions>();

  const commit: Partial<CommitOptions> = {};

  normalizeCommitBoolean(commit, 'all', argv.all);
  normalizeCommitBoolean(commit, 'noVerify', argv.noVerify);
  normalizeCommitBoolean(commit, 'push', argv.push);

  normalizeCommitString(commit, 'message', argv.commit);

  return commit;
}

function normalizeJobs(argv: CliArguments): Partial<JobOptions> {
  const normalizeJobBoolean = createBooleanNormalizer<JobOptions>();
  const normalizeJobString = createStringNormalizer<JobOptions>();

  const jobs: Partial<JobOptions> = {};

  normalizeJobBoolean(jobs, 'dryRun', argv.dryRun);
  normalizeJobString(jobs, 'only', argv.only);

  if (Array.isArray(argv.skip)) {
    jobs.skip = argv.skip.map(toString);
  }

  if (typeof argv.build === 'boolean') {
    jobs.build = argv.build;
  }

  if (typeof argv.publish === 'boolean') {
    jobs.publish = argv.publish;
  }

  if (typeof argv.test === 'boolean') {
    jobs.test = argv.test;
  }

  return jobs;
}

function createBooleanNormalizer<T>() {
  return function (
    options: Partial<T>,
    key: keyof PickByValue<Required<T>, boolean>,
    value: unknown
  ) {
    if (typeof value === 'boolean') {
      options[key] = value as never;
    }
  };
}

function createStringNormalizer<T>() {
  return function (
    options: Partial<T>,
    key: keyof PickByValue<Required<T>, string>,
    value: unknown
  ) {
    if (typeof value === 'string') {
      options[key] = value as never;
    }
  };
}

function toString(value: unknown) {
  return String(value);
}
