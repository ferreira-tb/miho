import { execa, type Options as ExecaOptions } from 'execa';
import type { MihoPackage } from '../files';
import type { CommitOptions, Nullish, PartialNullish } from '../types';
import { isNotBlank, CommitCommand, CommitDefaults } from '../utils';

interface Args {
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
    }

    // Should be the last.
    if (this.all) {
      commandArgs.push(CommitCommand.ALL);
    } else {
      packages.forEach((pkg) => {
        commandArgs.push(pkg.fullpath);
      });
    }

    await execa('git', ['commit', ...commandArgs], execaOptions);
  }

  public async pushCommit(args: PushCommitArgs) {
    const { execaOptions, dryRun } = args;

    const commandArgs = ['push'];
    if (dryRun) commandArgs.push(CommitCommand.DRY_RUN);

    await execa('git', commandArgs, execaOptions);
  }
}
