---
outline: [2, 3]
---

# Typescript

## Utility Types

### ExtractPartial\<T, K>

<<< ../../src/utils/types.ts#ExtractPartial

### ExtractRequired\<T, K>

<<< ../../src/utils/types.ts#ExtractRequired

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

### PickPartial<T, K>

<<< ../../src/utils/types.ts#PickPartial

Constructs a type by picking the set of properties `K` from a [`Partial`](https://www.typescriptlang.org/docs/handbook/utility-types.html#partialtype) version of `T`.

### PickRequired<T, K>

<<< ../../src/utils/types.ts#PickRequired

Constructs a type by picking the set of properties `K` from a [`Required`](https://www.typescriptlang.org/docs/handbook/utility-types.html#requiredtype) version of `T`.

### WithPartial\<T, K>

<<< ../../src/utils/types.ts#WithPartial

Constructs a type consisting of some properties of `T` set to partial.

### WithRequired\<T, K>

<<< ../../src/utils/types.ts#WithRequired

Constructs a type consisting of some properties of `T` set to required.
