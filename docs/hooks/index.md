---
outline: [2, 3]
---

# Hooks

## Bump lifecycle

```ts
interface HookCallbackParameters<T> {
  miho: Miho;
  data: T;
}
```

### afterAll

```ts
type HookAfterAllCallback = (
  data: HookCallbackParameters<PackageData[]>
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
  data: HookCallbackParameters<PackageData>
) => MaybePromise<void>;

interface Miho {
  afterEach(cb: MaybeArray<HookAfterEachCallback>): Miho;
}
```

Register one or more callbacks to be called after each [`bump()`](#bump).

### beforeAll

```ts
type HookBeforeAllCallback = (
  data: HookCallbackParameters<PackageData[]>
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
  data: HookCallbackParameters<PackageData>
) => MaybePromise<boolean | void>;

interface Miho {
  beforeEach(cb: MaybeArray<HookBeforeEachCallback>): Miho;
}
```

Register one or more callbacks to be called before each [`bump()`](#bump).

If `false` is returned, the operation will be aborted.
