import type { Miho } from '../miho';
import type { MihoHooks, MihoEventData } from '../types';

interface MihoEventInit<T extends keyof MihoHooks> {
  readonly miho: Miho;
  readonly data: MihoEventData<T>;
  readonly cancelable?: boolean;
}

export class MihoEvent<T extends keyof MihoHooks> extends Event {
  public declare readonly type: T;
  public readonly miho: Miho;
  public readonly data: MihoEventData<T>;

  constructor(type: T, eventInit: MihoEventInit<T>) {
    const { miho, data, ...init } = eventInit;
    super(type, init);

    this.miho = miho;
    this.data = data;
  }
}
