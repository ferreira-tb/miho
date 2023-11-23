import fs from 'node:fs/promises';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { defaultOptions } from './utils';
import {
  Miho,
  PackageData,
  type HookBeforeEachCallback,
  type HookAfterEachCallback,
  type HookBeforeAllCallback,
  type HookAfterAllCallback
} from '../src';
import {
  createMockPackages,
  getTempDir,
  MihoMock,
  PackageJsonMock
} from './utils';

beforeEach(createMockPackages);

expect.extend({
  async toHaveBeenBumped(oldPkgs: PackageData[]) {
    const updatedMiho = await new Miho(defaultOptions).search();
    const updatedPkgs = updatedMiho.getPackages();

    function bumped(pkg: PackageData) {
      const old = oldPkgs.find(({ name }) => name === pkg.name);
      if (!old) return false;
      return pkg.version === old.newVersion;
    }

    return {
      pass: updatedPkgs.every(bumped),
      message: () => {
        const bumpedAmount = updatedPkgs.filter(bumped).length;
        return `${bumpedAmount} package(s) bumped`;
      }
    };
  }
});

describe('Miho.prototype.search', () => {
  const temp = getTempDir();

  it('should find something', async () => {
    const miho = await new Miho(defaultOptions).search();
    const pkgs = miho.getPackages();
    expect(pkgs.length).toBeGreaterThanOrEqual(1);
  });

  it('should be recursive', async () => {
    const miho = await new Miho().search(defaultOptions);
    const ents = await fs.readdir(temp, { withFileTypes: true });

    expect(ents.filter(PackageJsonMock.isPackage)).toHaveLength(1);
    expect(miho.getPackages()).toHaveLength(MihoMock.DEFAULT_AMOUNT);
  });

  it('should not be recursive', async () => {
    // If the search is not recursive, --include is ignored.
    // Miho will only search the current working directory.
    const miho = await new Miho().search({
      ...defaultOptions,
      recursive: false
    });

    expect(miho.getPackages()).toHaveLength(0);
  });
});

describe('Miho.prototype.getPackages', () => {
  it('should find all packages', async () => {
    const miho = await new Miho().search(defaultOptions);
    expect(miho.getPackages()).toHaveLength(MihoMock.DEFAULT_AMOUNT);
  });

  it('should filter correctly', async () => {
    const miho = await new Miho().search(defaultOptions);
    const pkgs = miho.getPackages({
      filter: (pkg) => !pkg.name?.startsWith(MihoMock.PACKAGE_PREFIX)
    });

    expect(pkgs).toHaveLength(0);
  });
});

describe('Miho.prototype.bump', () => {
  it('should bump', async () => {
    const miho = await new Miho().search(defaultOptions);
    const pkgs = miho.getPackages();

    const results = await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(results.every(Boolean)).toBe(true);
    await expect(pkgs).toHaveBeenBumped();
  });

  it('should execute callback before', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeEachCallback = vi.fn(() => true);

    miho.beforeEach(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    expect(pkgs.length).toBeGreaterThanOrEqual(1);
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalledTimes(pkgs.length);
  });

  it('should abort if false is returned', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeEachCallback = vi.fn(() => false);

    miho.beforeEach(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalled();
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute callback after', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookAfterEachCallback = vi.fn(() => void 0);

    miho.beforeEach(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalledTimes(pkgs.length);
  });
});

describe('Miho.prototype.bumpAll', () => {
  it('should bump all', async () => {
    const miho = new Miho({
      ...defaultOptions,
      release: 'major'
    });

    await miho.search();
    const pkgs = miho.getPackages();

    const amount = await miho.bumpAll();
    expect(amount).toBe(pkgs.length);
    await expect(pkgs).toHaveBeenBumped();
  });

  it('should execute callback before', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeAllCallback = vi.fn(() => true);

    miho.beforeAll(cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
  });

  it('should abort if false is returned', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeAllCallback = vi.fn(() => false);

    miho.beforeAll(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute callback after', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookAfterAllCallback = vi.fn(() => void 0);

    miho.afterAll(cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
  });
});

describe('Miho.prototype.resolveHooks', () => {
  it('should resolve', async () => {
    const miho = new Miho(defaultOptions);
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
