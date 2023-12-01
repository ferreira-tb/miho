import { PackageManager } from './enum';

const npmPrefix = new RegExp(`^${PackageManager.NPM}`);
const pnpmPrefix = new RegExp(`^${PackageManager.PNPM}`);
const yarnPrefix = new RegExp(`^${PackageManager.YARN}`);

export const regex = {
  dirNodeModules: /node_modules/,
  dirGit: /\.git/,
  npmPrefix,
  npmLock: /package-lock\.json/,
  pnpmPrefix,
  pnpmLock: /pnpm-lock\.json/,
  yarnPrefix,
  yarnLock: /yarn\.lock/
} as const;
