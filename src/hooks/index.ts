import { HookListenerMap } from './map';
import type { MihoEvent } from './event';
import type { MihoHooks } from '../types';

export * from './event';

export class MihoEmitter {
  readonly #hookListenerMap = new HookListenerMap();

  /** Adds the listener function to the end of the listeners array for the hook named `hookName`. */
  public on<T extends keyof MihoHooks>(hookName: T, listener: MihoHooks[T]) {
    this.#hookListenerMap.set(hookName, listener);
    return this;
  }

  public off<T extends keyof MihoHooks>(hookName: T, listener: MihoHooks[T]) {
    this.#hookListenerMap.remove(hookName, listener);
    return this;
  }

  protected async executeHook<T extends keyof MihoHooks>(event: MihoEvent<T>) {
    const listeners = this.#hookListenerMap.get(event.type);
    for (const listener of listeners) {
      await listener(event as any);
      if (event.cancelable && event.defaultPrevented) {
        break;
      }
    }

    return event.defaultPrevented;
  }

  /** Register multiple listeners simultaneously. */
  protected resolveListeners<T extends keyof MihoHooks>(
    hooks: Partial<MihoHooks>
  ): this {
    Object.entries(hooks).forEach(([key, value]: [T, MihoHooks[T]]) => {
      this.#hookListenerMap.set(key, value);
    });

    return this;
  }

  /**
   * Removes all listeners associated with one or more hooks.
   * If no hook name is specified, listeners from all hooks will be removed.
   */
  public removeAllListeners<T extends keyof MihoHooks>(
    hookName?: T | T[]
  ): this {
    if (hookName) {
      const hooks = Array.isArray(hookName) ? hookName : [hookName];
      hooks.forEach((hook) => void this.#hookListenerMap.delete(hook));
    } else {
      this.#hookListenerMap.clear();
    }
    return this;
  }

  get addListener() {
    return this.on;
  }

  get removeListener() {
    return this.off;
  }
}
