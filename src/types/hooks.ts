import type { Miho, FileData } from '../index';
import type { MaybePromise, MaybeArray } from './utils';

export interface HookCallbackParameters<T> {
  miho: Miho;
  data: T;
}

// Bump lifecycle
export type HookAfterAllCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<void>;

export type HookAfterEachCallback = (
  data: HookCallbackParameters<FileData>
) => MaybePromise<void>;

export type HookBeforeAllCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<boolean | void>;

export type HookBeforeEachCallback = (
  data: HookCallbackParameters<FileData>
) => MaybePromise<boolean | void>;

// Commit lifecycle
export type HookAfterCommitCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<void>;

export type HookAfterPushCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<void>;

export type HookBeforeCommitCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<boolean | void>;

export type HookBeforePushCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<boolean | void>;

export type MihoHooks = {
  // Bump lifecycle
  readonly afterAll: MaybeArray<HookAfterAllCallback>;
  readonly afterEach: MaybeArray<HookAfterEachCallback>;
  readonly beforeAll: MaybeArray<HookBeforeAllCallback>;
  readonly beforeEach: MaybeArray<HookBeforeEachCallback>;

  // Commit lifecycle
  readonly afterCommit: MaybeArray<HookAfterCommitCallback>;
  readonly afterPush: MaybeArray<HookAfterPushCallback>;
  readonly beforeCommit: MaybeArray<HookBeforeCommitCallback>;
  readonly beforePush: MaybeArray<HookBeforePushCallback>;
};

// prettier-ignore
export type MihoHookCallback<T extends keyof MihoHooks> = 
  T extends 'afterAll' ? HookAfterAllCallback :
  T extends 'afterCommit' ? HookAfterCommitCallback :
  T extends 'afterEach' ? HookAfterEachCallback :
  T extends 'afterPush' ? HookAfterPushCallback :
  T extends 'beforeAll' ? HookBeforeAllCallback :
  T extends 'beforeEach' ? HookBeforeEachCallback :
  T extends 'beforeCommit' ? HookBeforeCommitCallback :
  T extends 'beforePush' ? HookBeforePushCallback :
  never
