import type { MihoHooks } from '../types';

/**
 * @internal
 * @ignore
 */
export class HookCallbackMap<T extends keyof MihoHooks> extends Map<
  T,
  MihoHooks[T]
> {
  public override get(key: T): MihoHooks[T] {
    return super.get(key) ?? [];
  }

  public override set(key: T, value: MihoHooks[T]) {
    const previous = this.get(key);
    const cbs = Array.isArray(value) ? value : [value];
    if (Array.isArray(previous)) {
      super.set(key, [...previous, ...cbs]);
    } else {
      super.set(key, [previous, ...cbs]);
    }

    return this;
  }
}
