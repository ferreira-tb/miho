---
outline: [2, 3]
---

# Typescript Utilities

## Utility Types

### MaybeArray

```ts
type MaybeArray<T> = T | T[];
```

Something may or may not be an array.

### MaybePromise

```ts
type MaybePromise<T> = T | Promise<T>;
```

Something may or may not be a promise.

### Nullable

```ts
type Nullable<T> = T | null | undefined;
```

Something may be nullish.
