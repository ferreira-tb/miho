import { execa } from 'execa';
import type { MihoPackage } from '../files';
import type { CommitOptions, PartialNullish } from '../types';
import { isNotBlank } from '../utils';
import { CommitCommand, CommitDefaults } from './enum';

export class GitCommit implements CommitOptions {
  public readonly all: boolean;
  public readonly message: string;
  public readonly 'no-verify': boolean;
  public readonly push: boolean;

  constructor(options: PartialNullish<CommitOptions> = {}) {
    this.message = isNotBlank(options.message)
      ? options.message
      : CommitDefaults.DEFAULT_MESSAGE;

    this.all = options.all ?? false;
    this['no-verify'] = options['no-verify'] ?? false;
    this.push = options.push ?? false;
  }

  public async commit(packages: MihoPackage[]) {
    const args: string[] = [CommitCommand.MESSAGE, this.message];

    if (this['no-verify']) {
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

    await execa('git', ['commit', ...args]);
    if (this.push) await execa('git', ['push']);
  }
}
