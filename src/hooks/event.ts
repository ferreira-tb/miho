import type { Miho } from '../miho';
import type { MihoEventData, MihoHooks } from '../types';

interface MihoEventInit<T extends keyof MihoHooks> {
  readonly cancelable?: boolean;
  readonly data: MihoEventData<T>;
  readonly miho: Miho;
}

export class MihoEvent<T extends keyof MihoHooks> extends Event {
  public readonly data: MihoEventData<T>;
  public readonly miho: Miho;
  public declare readonly type: T;

  constructor(type: T, eventInit: MihoEventInit<T>) {
    const { miho, data, ...init } = eventInit;
    super(type, init);

    this.miho = miho;
    this.data = data;
  }
}
