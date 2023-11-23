import fs from 'node:fs/promises';
import { beforeEach, describe, expect, it } from 'vitest';
import { Miho, type MihoOptions } from '../src';
import {
  createMockPackages,
  compareOldPackages,
  getTempDir,
  MihoMock,
  PackageJsonMock
} from './utils';

beforeEach(createMockPackages);

const include: MihoOptions['include'] = [MihoMock.TEMP_GLOB.toString()];
const filter: MihoOptions['filter'] = [/miho/];

const options: Partial<MihoOptions> = {
  include,
  filter,
  recursive: true
};

describe('Miho', () => {
  const temp = getTempDir();

  it('should init', async () => {
    const miho = await new Miho(options).search();
    expect(miho).toBeInstanceOf(Miho);
  });

  it('should be recursive', async () => {
    const miho = await new Miho(options).search();
    const ents = await fs.readdir(temp, { withFileTypes: true });

    expect(ents.filter(PackageJsonMock.isPackage)).toHaveLength(1);
    expect(miho.getPackages()).toHaveLength(MihoMock.DEFAULT_AMOUNT);
  });

  it('should NOT be recursive', async () => {
    // If the search is not recursive, --include is ignored.
    // Miho will only search the current working directory.
    const miho = await new Miho({ ...options, recursive: false }).search();
    expect(miho.getPackages()).toHaveLength(0);
  });
});

describe('Miho.prototype.getPackages', () => {
  it('should find all packages', async () => {
    const miho = await new Miho(options).search();
    expect(miho.getPackages()).toHaveLength(MihoMock.DEFAULT_AMOUNT);
  });

  it('should filter correctly', async () => {
    const miho = await new Miho(options).search();
    const pkgs = miho.getPackages({
      filter: (pkg) => !pkg.name?.startsWith(MihoMock.PACKAGE_PREFIX)
    });

    expect(pkgs).toHaveLength(0);
  });
});

describe('Miho.prototype.bump', () => {
  it('should bump', async () => {
    const miho = await new Miho({ ...options, release: 'major' }).search();
    const pkgs = miho.getPackages();
    await Promise.all(pkgs.map(({ id }) => miho.bump(id)));
    await compareOldPackages(pkgs, options);
  });
});

describe('Miho.prototype.bumpAll', () => {
  it('should bump all', async () => {
    const miho = await new Miho({ ...options, release: 'major' }).search();
    const pkgs = miho.getPackages();
    await miho.bumpAll();
    await compareOldPackages(pkgs, options);
  });
});
