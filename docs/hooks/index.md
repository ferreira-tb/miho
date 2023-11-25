---
outline: [2, 3]
---

# Hooks

```ts
miho.on('beforeEach', async (event) => {
  const { miho, data } = event;
  const result = await doSomethingAsync(miho, data);
  if (!result) event.preventDefault();
});
```

::: tip Aborting
Cancelable events can be aborted calling `event.preventDefault()`
:::

## Event

All listeners are called with an instance of `MihoEvent` as its first argument.

```ts
interface MihoEvent<T extends keyof MihoHooks> extends Event {
  readonly type: T;
  readonly miho: Miho;
  readonly data: MihoEventData<T>;
}
```

## Bump

### afterAll

| Data         | Cancelable |
| :----------- | :--------: |
| `FileData[]` |  `false`   |

Register a listener to be called after [`bumpAll()`](#bumpall).

### afterEach

| Data       | Cancelable |
| :--------- | :--------: |
| `FileData` |  `false`   |

Register a listener to be called after each [`bump()`](#bump).

### beforeAll

| Data         | Cancelable |
| :----------- | :--------: |
| `FileData[]` |   `true`   |

Register a listener to be called before [`bumpAll()`](#bumpall).

### beforeEach

| Data       | Cancelable |
| :--------- | :--------: |
| `FileData` |   `true`   |

Register a listener to be called before each [`bump()`](#bump).

## Commit

### afterCommit

| Data         | Cancelable |
| :----------- | :--------: |
| `FileData[]` |  `false`   |

Register a listener to be called after committing.

### afterPush

| Data         | Cancelable |
| :----------- | :--------: |
| `FileData[]` |  `false`   |

Register a listener to be called after pushing the commit.

### beforeCommit

| Data         | Cancelable |
| :----------- | :--------: |
| `FileData[]` |   `true`   |

Register a listener to be called before committing.

### beforePush

| Data         | Cancelable |
| :----------- | :--------: |
| `FileData[]` |   `true`   |

Register a listener to be called before pushing the commit.
