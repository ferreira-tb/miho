import path from 'node:path';
import fs from 'node:fs/promises';
import process from 'node:process';
import { regex } from './regex';
import { FileType, PackageManager } from './enum';

// #region DetectPackageManagerOptions
interface DetectPackageManagerOptions {
  /**
   * Current working directory.
   * @default process.cwd()
   */
  cwd?: string;
  /**
   * Default package manager.
   * @default 'npm'
   */
  default?: PackageManager;
}
// #endregion DetectPackageManagerOptions

const managerName: Record<PackageManager, RegExp> = {
  npm: regex.npmPrefix,
  pnpm: regex.pnpmPrefix,
  yarn: regex.yarnPrefix
};

const lockfile: Record<PackageManager, RegExp> = {
  npm: regex.npmLock,
  pnpm: regex.pnpmLock,
  yarn: regex.yarnLock
};

export async function detectPackageManager(
  options?: DetectPackageManagerOptions
): Promise<PackageManager> {
  const cwd = options?.cwd ?? process.cwd();
  const defaultManager = options?.default ?? PackageManager.NPM;

  let dirents = await fs.readdir(cwd, { withFileTypes: true });
  dirents = dirents.filter((ent) => ent.isFile());

  const packageJsonDirent = dirents.find(
    (ent) => ent.name === FileType.PACKAGE_JSON
  );

  if (packageJsonDirent) {
    const packageJsonPath = path.join(cwd, packageJsonDirent.name);
    const json = JSON.parse(
      await fs.readFile(packageJsonPath, 'utf-8')
    ) as Record<string, unknown>;

    if (typeof json.packageManager === 'string') {
      const pm = json.packageManager;
      if (managerName.yarn.test(pm)) return PackageManager.YARN;
      if (managerName.pnpm.test(pm)) return PackageManager.PNPM;
      if (managerName.npm.test(pm)) return PackageManager.NPM;
    }
  }

  for (const dirent of dirents) {
    const { name } = dirent;
    if (lockfile.yarn.test(name)) return PackageManager.YARN;
    if (lockfile.pnpm.test(name)) return PackageManager.PNPM;
    if (lockfile.npm.test(name)) return PackageManager.NPM;
  }

  return defaultManager;
}

export function isPackageManager(value: unknown): value is PackageManager {
  if (typeof value !== 'string' || value.length === 0) return false;
  return Object.values(PackageManager).some((pm) => pm === value);
}

export type { DetectPackageManagerOptions };
