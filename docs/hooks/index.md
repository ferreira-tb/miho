---
outline: [2, 3]
---

# Hooks

```ts
interface HookCallbackParameters<T> {
  miho: Miho;
  data: T;
}
```

## Bump

### afterAll

```ts
type HookAfterAllCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<void>;

interface Miho {
  afterAll(cb: MaybeArray<HookAfterAllCallback>): Miho;
}
```

Register one or more callbacks to be called after [`bumpAll()`](#bumpall).

```ts
miho.afterAll(async (data) => {
  await doSomethingAsync(data);
});
```

### afterEach

```ts
type HookAfterEachCallback = (
  data: HookCallbackParameters<FileData>
) => MaybePromise<void>;

interface Miho {
  afterEach(cb: MaybeArray<HookAfterEachCallback>): Miho;
}
```

Register one or more callbacks to be called after each [`bump()`](#bump).

### beforeAll

```ts
type HookBeforeAllCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<boolean | void>;

interface Miho {
  beforeAll(cb: MaybeArray<HookBeforeAllCallback>): Miho;
}
```

Register one or more callbacks to be called before [`bumpAll()`](#bumpall).

If `false` is returned, the operation will be aborted.

### beforeEach

```ts
type HookBeforeEachCallback = (
  data: HookCallbackParameters<FileData>
) => MaybePromise<boolean | void>;

interface Miho {
  beforeEach(cb: MaybeArray<HookBeforeEachCallback>): Miho;
}
```

Register one or more callbacks to be called before each [`bump()`](#bump).

If `false` is returned, the operation will be aborted.

## Commit

### afterCommit

```ts
type HookAfterCommitCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<void>;

interface Miho {
  afterCommit(cb: MaybeArray<HookAfterCommitCallback>): Miho;
}
```

Register one or more callbacks to be called after committing.

### afterPush

```ts
type HookAfterPushCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<void>;

interface Miho {
  afterPush(cb: MaybeArray<HookAfterPushCallback>): Miho;
}
```

Register one or more callbacks to be called after pushing the commit.

### beforeCommit

```ts
type HookBeforeCommitCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<boolean | void>;

interface Miho {
  beforeCommit(cb: MaybeArray<HookBeforeCommitCallback>): Miho;
}
```

Register one or more callbacks to be called before committing.

If `false` is returned, the operation will be aborted.

### beforePush

```ts
type HookBeforePushCallback = (
  data: HookCallbackParameters<FileData[]>
) => MaybePromise<boolean | void>;

interface Miho {
  beforePush(cb: MaybeArray<HookBeforePushCallback>): Miho;
}
```

Register one or more callbacks to be called before pushing the commit.

If `false` is returned, the operation will be aborted.
