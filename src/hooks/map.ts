import type { MihoHooks } from '../types';

/**
 * @internal
 * @ignore
 */
export class HookListenerMap<T extends keyof MihoHooks> extends Map {
  public override get(key: T): MihoHooks[T][] {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-return
    return super.get(key) ?? [];
  }

  public remove(key: T, value: MihoHooks[T]) {
    const previous = this.get(key);
    super.set(
      key,
      previous.filter((cb) => cb !== value)
    );

    return this;
  }

  public override set(key: T, value: MihoHooks[T]) {
    const previous = this.get(key);
    super.set(key, [...previous, value]);
    return this;
  }
}
