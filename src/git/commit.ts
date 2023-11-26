import { execa, type Options as ExecaOptions } from 'execa';
import type { MihoPackage } from '../files';
import type { CommitOptions, PartialNullish } from '../types';
import { isNotBlank } from '../utils';
import { CommitCommand, CommitDefaults } from './enum';

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

  public async commit(packages: MihoPackage[], execaOptions?: ExecaOptions) {
    const args: string[] = [CommitCommand.MESSAGE, this.message];

    if (this.noVerify) {
      args.push(CommitCommand.NO_VERIFY);
    }

    // Should be the last.
    if (this.all) {
      args.push(CommitCommand.ALL);
    } else {
      packages.forEach((pkg) => {
        args.push(pkg.fullpath);
      });
    }

    await execa('git', ['commit', ...args], execaOptions);
  }

  public async pushCommit(execaOptions?: ExecaOptions) {
    await execa('git', ['push'], execaOptions);
  }
}
