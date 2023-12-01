import { execa } from 'execa';
import type { Miho } from '../miho';
import type {
  CommitArgs,
  CommitOptions,
  HandleExceptionArgs,
  PushCommitArgs
} from './types';
import {
  CommitCommand,
  CommitDefaults,
  LogLevel,
  type MaybePromise,
  MihoJob,
  type PartialNullish,
  logDryRun
} from '../utils';

export class GitCommit implements CommitOptions {
  public readonly all: boolean;
  public readonly message:
    | string
    | ((miho: Miho) => MaybePromise<string | null>);
  public readonly noVerify: boolean;
  public readonly push: boolean;

  constructor(
    private readonly miho: Miho,
    options: PartialNullish<CommitOptions> = {}
  ) {
    if (typeof options.message === 'function') {
      this.message = options.message.bind(miho);
    } else {
      this.message = options.message ?? CommitDefaults.DEFAULT_MESSAGE;
    }

    this.all = options.all ?? false;
    this.noVerify = options.noVerify ?? false;
    this.push = options.push ?? false;
  }

  public async commit(args: CommitArgs) {
    const { miho, packages, execaOptions, dryRun } = args;
    const message = await this.#resolveMessage();
    const commandArgs: string[] = [CommitCommand.MESSAGE, message];

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
      for (const pkg of packages) {
        commandArgs.push(pkg.fullpath);
      }
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

  async #resolveMessage(): Promise<string> {
    if (typeof this.message === 'string') return this.message;
    return (await this.message(this.miho)) ?? CommitDefaults.DEFAULT_MESSAGE;
  }
}
