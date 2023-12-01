---
outline: [2, 3]
---

# Typescript

## Utility Types

### MaybeArray\<T>

<<< ../../src/utils/types.ts#MaybeArray

Something may or may not be an array.

### MaybePromise\<T>

<<< ../../src/utils/types.ts#MaybePromise

Something may or may not be a promise.

### Nullish\<T>

<<< ../../src/utils/types.ts#Nullish

Something may be nullish.

### PartialNullish\<T>

<<< ../../src/utils/types.ts#PartialNullish

Constructs a type where all properties of `T` may be nullish.

### PickByValue\<T, V>

<<< ../../src/utils/types.ts#PickByValue

```ts
interface Movie {
  title: string;
  description: string;
  stars: number;
}

type MovieWithoutStars = PickByValue<Movie, string>;

const movie: MovieWithoutStars = {
  title: 'Resident Evil',
  description: '2002 action horror film'
};
```

Like [`Pick`](https://www.typescriptlang.org/docs/handbook/utility-types.html#picktype-keys), but constructs the type based on the values.

### WithPartial\<T, K>

<<< ../../src/utils/types.ts#WithPartial

Constructs a type consisting of some properties of `T` set to partial.

### WithRequired\<T, K>

<<< ../../src/utils/types.ts#WithRequired

Constructs a type consisting of some properties of `T` set to required.
