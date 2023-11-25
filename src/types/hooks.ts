import type { FileData } from '../index';
import type { MaybePromise } from './utils';
import type { MihoEvent } from '../hooks/event';

export type MihoHookBumpLifecycle =
  | 'afterAll'
  | 'afterEach'
  | 'beforeAll'
  | 'beforeEach';

export type MihoHookCommitLifecycle =
  | 'afterCommit'
  | 'afterPush'
  | 'beforeCommit'
  | 'beforePush';

export type MihoHookName = MihoHookBumpLifecycle | MihoHookCommitLifecycle;

export type MihoHooks = {
  [K in MihoHookName]: (e: MihoEvent<K>) => MaybePromise<void>;
};

// prettier-ignore
export type MihoEventData<T extends MihoHookName> = 
  T extends 'afterAll' ? FileData[] :
  T extends 'afterCommit' ? FileData[] :
  T extends 'afterEach' ? FileData :
  T extends 'afterPush' ? FileData[] :
  T extends 'beforeAll' ? FileData[] :
  T extends 'beforeCommit' ? FileData[] :
  T extends 'beforeEach' ? FileData :
  T extends 'beforePush' ? FileData[] :
  never;
