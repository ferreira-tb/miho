/* eslint-disable @typescript-eslint/no-mixed-enums */
import path from 'node:path';
import fs from 'node:fs/promises';
import process from 'node:process';
import { type Dirent, existsSync as exists } from 'node:fs';
import { FileType } from '../../src/utils';

export enum MihoMock {
  DEFAULT_AMOUNT = 10,
  DEFAULT_VERSION = '1.0.0',
  PACKAGE_PREFIX = 'package-',
  TEMP_DIR = '.temp',
  TEMP_SUBDIR_PREFIX = 'subdir'
}

export class PackageJsonMock {
  public readonly name: string;
  public readonly version = MihoMock.DEFAULT_VERSION;

  private static counter = 0;

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

  public static isPackage(this: void, dirent: Dirent) {
    return dirent.name === FileType.PACKAGE_JSON;
  }
}

export async function createMockPackages(testName: string) {
  const temp = getTempDir(testName);
  if (exists(temp)) await fs.rm(temp, { recursive: true });
  await fs.mkdir(temp, { recursive: true });

  let cwd = temp;
  for (let i = 0; i < MihoMock.DEFAULT_AMOUNT; i++) {
    const pkg = new PackageJsonMock();
    const json = JSON.stringify(pkg, null, 0);

    const filePath = path.join(cwd, FileType.PACKAGE_JSON);
    await fs.writeFile(filePath, json, 'utf-8');

    cwd = path.join(cwd, `${MihoMock.TEMP_SUBDIR_PREFIX}${i}`);
    await fs.mkdir(cwd);
  }

  return () => fs.rm(temp, { recursive: true });
}

export function getTempDir(testName: string) {
  return path.join(process.cwd(), MihoMock.TEMP_DIR, testName);
}
