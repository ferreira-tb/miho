export type MaybeArray<T> = T | T[];
export type MaybePromise<T> = T | Promise<T>;
export type Nullish<T> = T | null | undefined;

export type PartialNullish<T> = {
  [P in keyof T]?: Nullish<T[P]>;
};

export type PickByValue<T, V> = {
  [P in keyof T as T[P] extends V ? P : never]: T[P];
};

export type WithRequired<T, K extends keyof T> = Omit<T, K> &
  Required<Pick<T, K>>;
