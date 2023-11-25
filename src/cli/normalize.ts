import type {
  CliArguments,
  PickByValue,
  MihoOptions,
  CommitOptions
} from '../types';

const normalizeArgvBoolean = createBooleanNormalizer<MihoOptions>();
const normalizeArgvString = createStringNormalizer<MihoOptions>();

export function normalize(argv: CliArguments): Partial<MihoOptions> {
  const options: Partial<MihoOptions> = {};
  options.commit = normalizeCommit(argv);

  if (argv._[0]) {
    options.release = argv._[0];
  }

  normalizeArgvBoolean(options, 'recursive', argv.recursive);
  normalizeArgvBoolean(options, 'silent', argv.silent);
  normalizeArgvBoolean(options, 'verbose', argv.verbose);

  normalizeArgvString(options, 'preid', argv.preid);

  if (Array.isArray(argv.exclude)) {
    options.exclude = argv.exclude.map((i) => i.toString());
  }

  if (Array.isArray(argv.filter)) {
    options.filter = argv.filter.map((i) => {
      const value = i.toString();
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
    options.include = argv.include.map((i) => i.toString());
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
  normalizeCommitBoolean(commit, 'no-verify', argv['no-verify']);
  normalizeCommitString(commit, 'message', argv.commit);

  return commit;
}

function createBooleanNormalizer<T>() {
  return function (
    options: Partial<T>,
    key: keyof PickByValue<Required<T>, boolean>,
    value: unknown
  ) {
    if (typeof value === 'boolean') {
      options[key] = value as any;
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
      options[key] = value as any;
    }
  };
}
