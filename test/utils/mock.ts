import path from 'node:path';
import fs from 'node:fs/promises';
import process from 'node:process';
import { existsSync as exists, type Dirent } from 'node:fs';
import { Filename } from '../../src/utils';

export enum MihoMock {
  DEFAULT_AMOUNT = 10,
  DEFAULT_VERSION = '1.0.0',
  PACKAGE_PREFIX = 'package-',
  PACKAGE_FILENAME = Filename.PACKAGE_JSON,
  TEMP_DIR = '.temp',
  TEMP_SUBDIR_PREFIX = 'subdir',
  TEMP_GLOB = '.temp/**'
}

export class PackageJsonMock {
  private static counter = 0;

  public readonly name: string;
  public readonly version = MihoMock.DEFAULT_VERSION;

  constructor(name?: string) {
    if (name) {
      this.name = name;
    } else {
      this.name = `${MihoMock.PACKAGE_PREFIX}${++PackageJsonMock.counter}`;
    }
  }

  public toJSON() {
    return {
      name: this.name,
      version: this.version
    };
  }

  public static isPackage(dirent: Dirent) {
    return dirent.name === MihoMock.PACKAGE_FILENAME;
  }
}

export async function createMockPackages() {
  const temp = getTempDir();
  if (exists(temp)) fs.rm(temp, { recursive: true });
  await fs.mkdir(temp);

  let cwd = temp;
  for (let i = 0; i < MihoMock.DEFAULT_AMOUNT; i++) {
    const pkg = new PackageJsonMock();
    const json = JSON.stringify(pkg, null, 0);

    const filePath = path.join(cwd, MihoMock.PACKAGE_FILENAME);
    await fs.writeFile(filePath, json, 'utf-8');

    cwd = path.join(cwd, `${MihoMock.TEMP_SUBDIR_PREFIX}${i}`);
    await fs.mkdir(cwd);
  }

  return () => fs.rm(temp, { recursive: true });
}

export function getTempDir() {
  return path.join(process.cwd(), MihoMock.TEMP_DIR);
}
