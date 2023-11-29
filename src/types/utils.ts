// #region MaybeArray
type MaybeArray<T> = T | T[];
// #endregion MaybeArray

// #region MaybePromise
type MaybePromise<T> = T | Promise<T>;
// #endregion MaybePromise

// #region Nullish
type Nullish<T> = T | null | undefined;
// #endregion Nullish

// #region PartialNullish
type PartialNullish<T> = {
  [P in keyof T]?: Nullish<T[P]>;
};
// #endregion PartialNullish

// #region PickByValue
type PickByValue<T, V> = {
  [P in keyof T as T[P] extends V ? P : never]: T[P];
};
// #endregion PickByValue

// #region WithPartial
type WithPartial<T, K extends keyof T> = Omit<T, K> & Partial<Pick<T, K>>;
// #endregion WithPartial

// #region WithRequired
type WithRequired<T, K extends keyof T> = Omit<T, K> & Required<Pick<T, K>>;
// #endregion WithRequired

export type {
  MaybeArray,
  MaybePromise,
  Nullish,
  PartialNullish,
  PickByValue,
  WithPartial,
  WithRequired
};
