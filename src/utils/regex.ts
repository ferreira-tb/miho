export const regex = {
  dirNodeModules: /node_modules/,
  dirGit: /\.git/,
  npmLock: /package-lock\.json/,
  pnpmLock: /pnpm-lock\.json/,
  yarnLock: /yarn\.lock/
} as const;
