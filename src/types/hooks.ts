import type { Miho, FileData } from '../index';
import type { MaybePromise, MaybeArray } from './utils';

export interface HookCallbackParameters<T> {
  miho: Miho;
  data: T;
}

export type HookBeforeAllCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<boolean | void>;

export type HookAfterAllCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<void>;

export type HookBeforeEachCallback = (
  data: HookCallbackParameters<FileData>
) => MaybePromise<boolean | void>;

export type HookAfterEachCallback = (
  data: HookCallbackParameters<FileData>
) => MaybePromise<void>;

export type MihoHooks = {
  readonly beforeAll: MaybeArray<HookBeforeAllCallback>;
  readonly afterAll: MaybeArray<HookAfterAllCallback>;
  readonly beforeEach: MaybeArray<HookBeforeEachCallback>;
  readonly afterEach: MaybeArray<HookAfterEachCallback>;
};

export type MihoHookCallback<T extends keyof MihoHooks> = T extends 'beforeAll'
  ? HookBeforeAllCallback
  : T extends 'afterAll'
    ? HookAfterAllCallback
    : T extends 'beforeEach'
      ? HookBeforeEachCallback
      : T extends 'afterEach'
        ? HookAfterEachCallback
        : never;
