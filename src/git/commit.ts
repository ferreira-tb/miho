import { execa, type Options as ExecaOptions } from 'execa';
import { logDryRun, MihoJob } from '../utils';
import type { MihoPackage } from '../files';
import type { CommitOptions, Nullish, PartialNullish } from '../types';
import { isNotBlank, CommitCommand, CommitDefaults } from '../utils';
import type { Miho } from '../miho';

interface Args {
  miho: Miho,
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
    const { packages, execaOptions, dryRun } = args;
    const commandArgs: string[] = [CommitCommand.MESSAGE, this.message];

    if (this.noVerify) {
      commandArgs.push(CommitCommand.NO_VERIFY);
    }

    if (dryRun) {
      commandArgs.push(CommitCommand.DRY_RUN);
      logDryRun(args.miho, MihoJob.COMMIT);
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
      this.#handleException(err, dryRun);
    }
  }

  public async pushCommit(args: PushCommitArgs) {
    const { execaOptions, dryRun } = args;

    const commandArgs = ['push'];
    if (dryRun) {
      commandArgs.push(CommitCommand.DRY_RUN);
      logDryRun(args.miho, MihoJob.PUSH);
    }

    try {
      await execa('git', commandArgs, execaOptions);
    } catch (err) {
      this.#handleException(err, dryRun);
    }
  }

  #handleException(err: unknown, dryRun: Nullish<boolean>) {
    if (!(err instanceof Error)) return;
    if (!dryRun) throw err;
    console.error(err);
  }
}
