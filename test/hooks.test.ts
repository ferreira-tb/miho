import { beforeEach, describe, expect, it, vi } from 'vitest';
import {
  createMockPackages,
  getDefaultOptions,
  toHaveBeenBumped
} from './utils';
import {
  Miho,
  type HookBeforeEachCallback,
  type HookAfterEachCallback,
  type HookBeforeAllCallback,
  type HookAfterAllCallback
} from '../src';

const testName = 'hooks';
beforeEach(() => createMockPackages(testName));

expect.extend({
  toHaveBeenBumped: toHaveBeenBumped(testName, this)
});

describe('Miho.prototype.resolveHooks', () => {
  const options = getDefaultOptions(testName);

  it('should resolve', async () => {
    const miho = new Miho(options);
    const beforeEachCb: HookBeforeEachCallback = vi.fn(() => true);
    const afterEachCb: HookAfterEachCallback = vi.fn(() => void 0);
    const beforeAllCb: HookBeforeAllCallback = vi.fn(() => true);
    const afterAllCb: HookAfterAllCallback = vi.fn(() => void 0);

    miho.resolveHooks({
      beforeEach: beforeEachCb,
      afterEach: afterEachCb,
      beforeAll: [beforeAllCb, () => true],
      afterAll: [afterAllCb, () => void 0]
    });

    await miho.search();
    await miho.bumpAll();

    expect(beforeEachCb).toHaveBeenCalled();
    expect(afterEachCb).toHaveBeenCalled();
    expect(beforeAllCb).toHaveBeenCalled();
    expect(afterAllCb).toHaveBeenCalled();
  });
});

describe('Miho.prototype.clearHooks', () => {
  const options = getDefaultOptions(testName);

  it('should remove the callback', async () => {
    const miho = new Miho(options);
    const beforeEachCb: HookBeforeEachCallback = vi.fn(() => true);
    const afterEachCb: HookAfterEachCallback = vi.fn(() => void 0);

    miho.resolveHooks({
      beforeEach: beforeEachCb,
      afterEach: afterEachCb
    });

    miho.clearHooks('afterEach');

    await miho.search();
    await miho.bumpAll();

    expect(beforeEachCb).toHaveBeenCalled();
    expect(afterEachCb).not.toHaveBeenCalled();
  });

  it('should remove many callbacks', async () => {
    const miho = new Miho(options);
    const beforeEachCb: HookBeforeEachCallback = vi.fn(() => true);
    const afterEachCb: HookAfterEachCallback = vi.fn(() => void 0);
    const beforeAllCb: HookBeforeAllCallback = vi.fn(() => true);
    const afterAllCb: HookAfterAllCallback = vi.fn(() => void 0);

    miho.resolveHooks({
      beforeEach: beforeEachCb,
      afterEach: afterEachCb,
      beforeAll: beforeAllCb,
      afterAll: afterAllCb
    });

    miho.clearHooks(['afterEach', 'beforeAll', 'afterAll']);

    await miho.search();
    await miho.bumpAll();

    expect(beforeEachCb).toHaveBeenCalled();
    expect(afterEachCb).not.toHaveBeenCalled();
    expect(beforeAllCb).not.toHaveBeenCalled();
    expect(afterAllCb).not.toHaveBeenCalled();
  });
});

describe('Miho.prototype.clearHooks', () => {
  const options = getDefaultOptions(testName);

  it('should remove all callbacks', async () => {
    const miho = new Miho(options);
    const beforeEachCb: HookBeforeEachCallback = vi.fn(() => true);
    const afterEachCb: HookAfterEachCallback = vi.fn(() => void 0);
    const beforeAllCb: HookBeforeAllCallback = vi.fn(() => true);
    const afterAllCb: HookAfterAllCallback = vi.fn(() => void 0);

    miho.resolveHooks({
      beforeEach: beforeEachCb,
      afterEach: afterEachCb,
      beforeAll: beforeAllCb,
      afterAll: afterAllCb
    });

    miho.clearAllHooks();

    await miho.search();
    await miho.bumpAll();

    expect(beforeEachCb).not.toHaveBeenCalled();
    expect(afterEachCb).not.toHaveBeenCalled();
    expect(beforeAllCb).not.toHaveBeenCalled();
    expect(afterAllCb).not.toHaveBeenCalled();
  });
});
