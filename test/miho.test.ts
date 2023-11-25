import fs from 'node:fs/promises';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import {
  Miho,
  type HookBeforeEachCallback,
  type HookAfterEachCallback,
  type HookBeforeAllCallback,
  type HookAfterAllCallback
} from '../src';
import {
  createMockPackages,
  getTempDir,
  getDefaultOptions,
  toHaveBeenBumped,
  MihoMock,
  PackageJsonMock
} from './utils';

const testName = 'miho';
beforeEach(() => createMockPackages(testName));

expect.extend({
  toHaveBeenBumped: toHaveBeenBumped(testName, this)
});

describe('Miho.prototype.search', () => {
  const temp = getTempDir(testName);
  const options = getDefaultOptions(testName);

  it('should find something', async () => {
    const miho = await new Miho(options).search();
    const pkgs = miho.getPackages();
    expect(pkgs.length).toBeGreaterThanOrEqual(1);
  });

  it('should be recursive', async () => {
    const miho = await new Miho().search(options);
    const ents = await fs.readdir(temp, { withFileTypes: true });

    expect(ents.filter(PackageJsonMock.isPackage)).toHaveLength(1);
    expect(miho.getPackages()).toHaveLength(MihoMock.DEFAULT_AMOUNT);
  });

  it('should not be recursive', async () => {
    // If the search is not recursive, --include is ignored.
    // Miho will only search the current working directory.
    const miho = await new Miho().search({
      ...options,
      recursive: false
    });

    expect(miho.getPackages()).toHaveLength(0);
  });
});

describe('Miho.prototype.getPackages', () => {
  const options = getDefaultOptions(testName);

  it('should find all packages', async () => {
    const miho = await new Miho().search(options);
    expect(miho.getPackages()).toHaveLength(MihoMock.DEFAULT_AMOUNT);
  });

  it('should filter correctly', async () => {
    const miho = await new Miho().search(options);
    const pkgs = miho.getPackages({
      filter: (pkg) => !pkg.name?.startsWith(MihoMock.PACKAGE_PREFIX)
    });

    expect(pkgs).toHaveLength(0);
  });
});

describe('Miho.prototype.getPackageByName', () => {
  const options = getDefaultOptions(testName);

  it('should find', async () => {
    const miho = await new Miho().search(options);
    const pkgs = miho.getPackages();
    const packageName = pkgs[2].name;
    if (!packageName) {
      throw new TypeError('No package name to search for.');
    }

    const pkg = miho.getPackageByName(packageName);
    expect(pkg).toBeTruthy();
  });

  it('should not find', async () => {
    const miho = await new Miho().search(options);
    const pkg = miho.getPackageByName('awesome-miho-explosion');
    expect(pkg).toBeNull();
  });
});

describe('Miho.prototype.bump', () => {
  const options = getDefaultOptions(testName);

  it('should bump', async () => {
    const miho = await new Miho().search(options);
    const pkgs = miho.getPackages();

    const results = await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(results.every(Boolean)).toBe(true);
    await expect(pkgs).toHaveBeenBumped();
  });

  it('should execute callback before', async () => {
    const miho = new Miho(options);
    const cb: HookBeforeEachCallback = vi.fn(() => true);

    miho.beforeEach(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    expect(pkgs.length).toBeGreaterThanOrEqual(1);
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalledTimes(pkgs.length);
  });

  it('should abort if false is returned', async () => {
    const miho = new Miho(options);
    const cb: HookBeforeEachCallback = vi.fn(() => false);

    miho.beforeEach(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalled();
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute callback after', async () => {
    const miho = new Miho(options);
    const cb: HookAfterEachCallback = vi.fn(() => void 0);

    miho.beforeEach(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(cb).toHaveBeenCalledTimes(pkgs.length);
  });
});

describe('Miho.prototype.bumpAll', () => {
  const options = getDefaultOptions(testName);

  it('should bump all', async () => {
    const miho = new Miho({
      ...options,
      release: 'major'
    });

    await miho.search();
    const pkgs = miho.getPackages();

    const amount = await miho.bumpAll();
    expect(amount).toBe(pkgs.length);
    await expect(pkgs).toHaveBeenBumped();
  });

  it('should execute callback before', async () => {
    const miho = new Miho(options);
    const cb: HookBeforeAllCallback = vi.fn(() => true);

    miho.beforeAll(cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
  });

  it('should abort if false is returned', async () => {
    const miho = new Miho(options);
    const cb: HookBeforeAllCallback = vi.fn(() => false);

    miho.beforeAll(cb);
    await miho.search();
    const pkgs = miho.getPackages();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute callback after', async () => {
    const miho = new Miho(options);
    const cb: HookAfterAllCallback = vi.fn(() => void 0);

    miho.afterAll(cb);
    await miho.search();
    await miho.bumpAll();

    expect(cb).toHaveBeenCalled();
  });
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

describe('Miho.prototype.l', () => {
  const options = getDefaultOptions(testName);

  it('should not explode', () => {
    const miho = new Miho(options);
    const spy = vi.spyOn(miho, 'l').mockImplementation(() => void 0);

    miho.l`LOG`;

    expect(spy).toHaveBeenCalled();
    expect(spy).toHaveBeenLastCalledWith(['LOG']);
    spy.mockReset();
  });
});
