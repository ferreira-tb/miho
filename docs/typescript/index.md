---
outline: [2, 3]
---

# Typescript

## Utility Types

### MaybeArray\<T>

<<< ../../src/types/utils.ts#MaybeArray

Something may or may not be an array.

### MaybePromise\<T>

<<< ../../src/types/utils.ts#MaybePromise

Something may or may not be a promise.

### Nullish\<T>

<<< ../../src/types/utils.ts#Nullish

Something may be nullish.

### PartialNullish\<T>

<<< ../../src/types/utils.ts#PartialNullish

Constructs a type where all properties of `T` may be nullish.

### PickByValue\<T, V>

<<< ../../src/types/utils.ts#PickByValue

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

<<< ../../src/types/utils.ts#WithPartial

Constructs a type consisting of some properties of `T` set to partial.

### WithRequired\<T, K>

<<< ../../src/types/utils.ts#WithRequired

Constructs a type consisting of some properties of `T` set to required.
