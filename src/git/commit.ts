import { execa, type Options as ExecaOptions } from 'execa';
import { logDryRun, LogLevel, MihoJob } from '../utils';
import type { MihoPackage } from '../files';
import type { CommitOptions, Nullish, PartialNullish } from '../types';
import { isNotBlank, CommitCommand, CommitDefaults } from '../utils';
import type { Miho } from '../miho';

interface Args {
  miho: Miho;
  execaOptions?: ExecaOptions;
  dryRun?: Nullish<boolean>;
}

interface CommitArgs extends Args {
  packages: MihoPackage[];
}

interface PushCommitArgs extends Args {
  /** @see https://git-scm.com/docs/git-push#Documentation/git-push.txt---dry-run */
  dryRun?: Nullish<boolean>;
}

type HandleExceptionArgs = Required<Omit<Args, 'execaOptions'>>;

export class GitCommit implements CommitOptions {
  public readonly all: boolean;
  public readonly message: string;
  public readonly noVerify: boolean;
  public readonly push: boolean;

  constructor(options: PartialNullish<CommitOptions> = {}) {
    this.message = isNotBlank(options.message)
      ? options.message
      : CommitDefaults.DEFAULT_MESSAGE;

    this.all = options.all ?? false;
    this.noVerify = options.noVerify ?? false;
    this.push = options.push ?? false;
  }

  public async commit(args: CommitArgs) {
    const { miho, packages, execaOptions, dryRun } = args;
    const commandArgs: string[] = [CommitCommand.MESSAGE, this.message];

    if (this.noVerify) {
      commandArgs.push(CommitCommand.NO_VERIFY);
    }

    if (dryRun) {
      commandArgs.push(CommitCommand.DRY_RUN);
      logDryRun(miho, MihoJob.COMMIT);
    }

    // Should be the last.
    if (this.all) {
      commandArgs.push(CommitCommand.ALL);
    } else {
      packages.forEach((pkg) => {
        commandArgs.push(pkg.fullpath);
      });
    }

    try {
      await execa('git', ['commit', ...commandArgs], execaOptions);
    } catch (err) {
      this.#handleException(err, { miho, dryRun });
    }
  }

  public async pushCommit(args: PushCommitArgs) {
    const { miho, execaOptions, dryRun } = args;

    const commandArgs = ['push'];
    if (dryRun) {
      commandArgs.push(CommitCommand.DRY_RUN);
      logDryRun(miho, MihoJob.PUSH);
    }

    try {
      await execa('git', commandArgs, execaOptions);
    } catch (err) {
      this.#handleException(err, { miho, dryRun });
    }
  }

  #handleException(err: unknown, args: HandleExceptionArgs) {
    if (!(err instanceof Error)) return;

    const { miho, dryRun } = args;
    if (!dryRun) throw err;
    miho.l(LogLevel.LOW)`${err}`;
  }
}
