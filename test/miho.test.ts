import fs from 'node:fs/promises';
import { beforeEach, describe, expect, it, vi } from 'vitest';
import { Miho, FileData } from '../src';
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

  it('should return miho', async () => {
    const miho = await new Miho(options).search();
    expect(miho).toBeInstanceOf(Miho);
  });

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

  it('should return FileData array', async () => {
    const miho = await new Miho().search(options);
    const packages = miho.getPackages();

    expect(Array.isArray(packages)).toBe(true);
    expect(packages.every((pkg) => pkg instanceof FileData)).toBe(true);
  });

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
    expect(pkg).toBeInstanceOf(FileData);
  });

  it('should find with regex', async () => {
    const miho = await new Miho().search(options);
    const pkg = miho.getPackageByName(
      new RegExp(`^${MihoMock.PACKAGE_PREFIX}`)
    );
    expect(pkg).toBeInstanceOf(FileData);
  });

  it('should not find', async () => {
    const miho = await new Miho().search(options);
    const pkg = miho.getPackageByName('awesome-miho-explosion');
    expect(pkg).toBeNull();
  });
});

describe('Miho.prototype.bump', () => {
  const options = getDefaultOptions(testName);

  it('should return boolean', async () => {
    const miho = await new Miho().search(options);
    const pkgs = miho.getPackages();

    const result = await miho.bump(pkgs[0].id);
    expect(result).toBeTypeOf('boolean');
  });

  it('should bump', async () => {
    const miho = await new Miho().search(options);
    const pkgs = miho.getPackages();

    const results = await Promise.all(pkgs.map(({ id }) => miho.bump(id)));

    expect(results.every(Boolean)).toBe(true);
    await expect(pkgs).toHaveBeenBumped();
  });
});

describe('Miho.prototype.bumpAll', () => {
  const options = getDefaultOptions(testName);

  it('should return integer', async () => {
    const miho = await new Miho(options).search();
    const amount = await miho.bumpAll();

    expect(amount).toBeTypeOf('number');
    expect(Number.isInteger(amount)).toBe(true);
  });

  it('should bump all', async () => {
    const miho = await new Miho(options).search();
    const pkgs = miho.getPackages();

    const amount = await miho.bumpAll();
    expect(amount).toBe(pkgs.length);
    await expect(pkgs).toHaveBeenBumped();
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
