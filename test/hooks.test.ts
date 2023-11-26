import { beforeEach, describe, expect, it, vi } from 'vitest';
import { Miho, FileData, type MihoEvent } from '../src';
import {
  createMockPackages,
  getDefaultOptions,
  toHaveBeenBumped
} from './utils';

const testName = 'hooks';
beforeEach(() => createMockPackages(testName));

expect.extend({
  toHaveBeenBumped: toHaveBeenBumped(testName, this)
});

describe('Miho.prototype.constructor', () => {
  const options = getDefaultOptions(testName);

  it('should resolve hooks', async () => {
    const beforeEach = vi.fn(() => void 0);
    const afterEach = vi.fn(() => void 0);
    const beforeAll = vi.fn(() => void 0);
    const afterAll = vi.fn(() => void 0);

    const miho = new Miho({
      hooks: {
        beforeEach,
        afterEach,
        beforeAll,
        afterAll
      }
    });

    await miho.search(options);
    await miho.bumpAll();

    expect(beforeEach).toHaveBeenCalled();
    expect(afterEach).toHaveBeenCalled();
    expect(beforeAll).toHaveBeenCalled();
    expect(afterAll).toHaveBeenCalled();
  });
});

describe('Miho.prototype.on', () => {
  const options = getDefaultOptions(testName);

  it('should return miho', () => {
    const miho = new Miho(options);
    const returnValue = miho.on('beforeEach', () => void 0);
    expect(returnValue).toBeInstanceOf(Miho);
  });
});

describe('Miho.prototype.on [BUMP]', () => {
  const options = getDefaultOptions(testName);

  it('should execute listener before', async () => {
    const miho = new Miho(options);
    const cb = vi.fn(() => void 0);

    miho.on('beforeEach', cb);
    await miho.search();
    const pkgs = miho.getPackages();
    expect(pkgs.length).toBeGreaterThanOrEqual(1);
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalledTimes(pkgs.length);
  });

  it('should emit "before" event correctly', async () => {
    const miho = new Miho(options);
    const cb = vi.fn((event: MihoEvent<'beforeEach'>) => {
      if (!(event.miho instanceof Miho)) {
        throw new TypeError('event.miho is not a Miho instance');
      } else if (!(event.data instanceof FileData)) {
        throw new TypeError('event.data is not FileData');
      }
    });

    miho.on('beforeEach', cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveReturned();
  });

  it('should abort if default is prevented', async () => {
    const miho = new Miho(options);
    const cb = vi.fn((e) => e.preventDefault());

    miho.on('beforeEach', cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalled();
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute listener after', async () => {
    const miho = new Miho(options);
    const cb = vi.fn(() => void 0);

    miho.on('afterEach', cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalledTimes(pkgs.length);
  });

  it('should emit "after" event correctly', async () => {
    const miho = new Miho(options);
    const cb = vi.fn((event: MihoEvent<'afterEach'>) => {
      if (!(event.miho instanceof Miho)) {
        throw new TypeError('event.miho is not a Miho instance');
      } else if (!(event.data instanceof FileData)) {
        throw new TypeError('event.data is not FileData');
      }
    });

    miho.on('afterEach', cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveReturned();
  });
});

describe('Miho.prototype.on [BUMP ALL]', () => {
  const options = getDefaultOptions(testName);

  it('should execute listener before', async () => {
    const miho = new Miho(options);
    const cb = vi.fn(() => void 0);

    miho.on('beforeAll', cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
  });

  it('should emit "before" event correctly', async () => {
    const miho = new Miho(options);
    const cb = vi.fn((event: MihoEvent<'beforeAll'>) => {
      if (!(event.miho instanceof Miho)) {
        throw new TypeError('event.miho is not a Miho instance');
      } else if (!event.data.every((f) => f instanceof FileData)) {
        throw new TypeError('event.data is not FileData[]');
      }
    });

    miho.on('beforeAll', cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveReturned();
  });

  it('should abort if default is prevented', async () => {
    const miho = new Miho(options);
    const cb = vi.fn((e: MihoEvent<'beforeAll'>) => e.preventDefault());

    miho.on('beforeAll', cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute listener after', async () => {
    const miho = new Miho(options);
    const cb = vi.fn(() => void 0);

    miho.on('afterAll', cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
  });

  it('should emit "after" event correctly', async () => {
    const miho = new Miho(options);
    const cb = vi.fn((event: MihoEvent<'afterAll'>) => {
      if (!(event.miho instanceof Miho)) {
        throw new TypeError('event.miho is not a Miho instance');
      } else if (!event.data.every((f) => f instanceof FileData)) {
        throw new TypeError('event.data is not FileData[]');
      }
    });

    miho.on('afterAll', cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveReturned();
  });
});

describe('Miho.prototype.off', () => {
  const options = getDefaultOptions(testName);

  it('should return miho', () => {
    const miho = new Miho(options);
    const returnValue = miho.off('beforeEach', () => void 0);
    expect(returnValue).toBeInstanceOf(Miho);
  });

  it('should remove listener', async () => {
    const miho = new Miho(options);
    const cb = vi.fn(() => void 0);
    const cb2 = vi.fn(() => void 0);

    miho.on('beforeEach', cb);
    miho.on('beforeEach', cb2);
    miho.off('beforeEach', cb2);

    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
    expect(cb2).not.toHaveBeenCalled();
  });
});

describe('Miho.prototype.addListener', () => {
  const options = getDefaultOptions(testName);

  it('should return miho', () => {
    const miho = new Miho(options);
    const returnValue = miho.addListener('beforeEach', () => void 0);
    expect(returnValue).toBeInstanceOf(Miho);
  });
});

describe('Miho.prototype.removeListener', () => {
  const options = getDefaultOptions(testName);

  it('should return miho', () => {
    const miho = new Miho(options);
    const returnValue = miho.removeListener('beforeEach', () => void 0);
    expect(returnValue).toBeInstanceOf(Miho);
  });
});

describe('Miho.prototype.removeAllListeners', () => {
  const options = getDefaultOptions(testName);

  it('should return miho', () => {
    const miho = new Miho(options);
    const returnValue = miho.removeAllListeners();

    expect(returnValue).toBeInstanceOf(Miho);
  });

  it('should remove listeners from a hook', async () => {
    const miho = new Miho(options);
    const beforeEach = vi.fn(() => void 0);
    const afterEach = vi.fn(() => void 0);

    miho.on('beforeEach', beforeEach);
    miho.on('afterEach', afterEach);

    miho.removeAllListeners('afterEach');

    await miho.search();
    await miho.bumpAll();

    expect(beforeEach).toHaveBeenCalled();
    expect(afterEach).not.toHaveBeenCalled();
  });

  it('should remove listeners from some hooks', async () => {
    const miho = new Miho(options);
    const beforeEach = vi.fn(() => void 0);
    const afterEach = vi.fn(() => void 0);
    const beforeAll = vi.fn(() => void 0);
    const afterAll = vi.fn(() => void 0);

    miho.on('beforeEach', beforeEach);
    miho.on('afterEach', afterEach);
    miho.on('beforeAll', beforeAll);
    miho.on('afterAll', afterAll);

    miho.removeAllListeners(['beforeAll', 'afterAll']);

    await miho.search();
    await miho.bumpAll();

    expect(beforeEach).toHaveBeenCalled();
    expect(afterEach).toHaveBeenCalled();
    expect(beforeAll).not.toHaveBeenCalled();
    expect(afterAll).not.toHaveBeenCalled();
  });

  it('should remove all listeners', async () => {
    const miho = new Miho(options);
    const beforeEach = vi.fn(() => void 0);
    const afterEach = vi.fn(() => void 0);
    const beforeAll = vi.fn(() => void 0);
    const afterAll = vi.fn(() => void 0);

    miho.on('beforeEach', beforeEach);
    miho.on('afterEach', afterEach);
    miho.on('beforeAll', beforeAll);
    miho.on('afterAll', afterAll);

    miho.removeAllListeners();

    await miho.search();
    await miho.bumpAll();

    expect(beforeEach).not.toHaveBeenCalled();
    expect(afterEach).not.toHaveBeenCalled();
    expect(beforeAll).not.toHaveBeenCalled();
    expect(afterAll).not.toHaveBeenCalled();
  });
});