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

describe('Miho', () => {
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
    const miho = await new Miho({
      ...defaultOptions,
      recursive: false
    }).search();
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
    const miho = await new Miho().search({
      ...defaultOptions,
      release: 'major'
    });
    const pkgs = miho.getPackages();

    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));
    await expect(pkgs).toHaveBeenBumped();
  });

  it('should execute callback before', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeEachCallback = () => true;

    const spy = vi.fn(cb);
    miho.beforeEach(spy);

    await miho.search();
    const pkgs = miho.getPackages();
    expect(pkgs.length).toBeGreaterThanOrEqual(1);

    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));
    expect(spy).toHaveBeenCalledTimes(pkgs.length);
  });

  it('should abort if false is returned', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeEachCallback = () => false;

    const spy = vi.fn(cb);
    miho.beforeEach(spy);

    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute callback after', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookAfterEachCallback = () => void 0;

    const spy = vi.fn(cb);
    miho.beforeEach(spy);

    await miho.search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));
    expect(spy).toHaveBeenCalledTimes(pkgs.length);
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

    await miho.bumpAll();
    await expect(pkgs).toHaveBeenBumped();
  });

  it('should execute callback before', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeAllCallback = () => true;

    const spy = vi.fn(cb);
    miho.beforeAll(spy);

    await miho.search();
    await miho.bumpAll();
    expect(spy).toHaveBeenCalledTimes(1);
  });

  it('should abort if false is returned', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookBeforeAllCallback = () => false;

    const spy = vi.fn(cb);
    miho.beforeAll(spy);

    await miho.search();
    const pkgs = miho.getPackages();
    await miho.bumpAll();
    await expect(pkgs).not.toHaveBeenBumped();
  });

  it('should execute callback after', async () => {
    const miho = new Miho(defaultOptions);
    const cb: HookAfterAllCallback = () => void 0;

    const spy = vi.fn(cb);
    miho.afterAll(spy);

    await miho.search();
    await miho.bumpAll();
    expect(spy).toHaveBeenCalledTimes(1);
  });
});
