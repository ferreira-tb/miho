import type { CommitOptions, PartialNullish } from '../types';
import { isNotBlank } from '../utils';

export class Commit implements CommitOptions {
  public static readonly DEFAULT_MESSAGE = 'chore: bump version';

  public readonly message: string;
  public readonly all: boolean;
  public readonly verify: boolean;

  constructor(options: PartialNullish<CommitOptions> = {}) {
    this.message = isNotBlank(options.message)
      ? options.message
      : Commit.DEFAULT_MESSAGE;

    this.all = options.all ?? false;
    this.verify = options.verify ?? true;
  }

  public commit() {
    console.log('TODO');
  }
}
