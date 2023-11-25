---
outline: [2, 3]
---

# Typescript

## Utility Types

### MaybeArray\<T>

```ts
type MaybeArray<T> = T | T[];
```

Something may or may not be an array.

### MaybePromise\<T>

```ts
type MaybePromise<T> = T | Promise<T>;
```

Something may or may not be a promise.

### Nullish\<T>

```ts
type Nullish<T> = T | null | undefined;
```

Something may be nullish.

### PartialNullish\<T>

```ts
type PartialNullish<T> = {
  [P in keyof T]?: Nullish<T[P]>;
};
```

Constructs a type where all properties of `T` maybe be nullish.

### PickByValue\<T, V>

```ts
type PickByValue<T, V> = {
  [P in keyof T as T[P] extends V ? P : never]: T[P];
};

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

### WithRequired\<T>

```ts
type WithRequired<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;
```

Constructs a type consisting of some properties of `T` set to required.
