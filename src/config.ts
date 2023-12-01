import process from 'node:process';
import { loadConfig as load } from 'c12';
import { PackageManager } from './utils';
import type { MihoOptions } from './miho';

/**
 * @internal
 * @ignore
 */
export const defaultConfig: MihoOptions = {
  exclude: [],
  filter: [],
  overrides: {},
  packageManager: PackageManager.NPM,
  preid: 'alpha',
  recursive: false,
  release: 'patch',
  silent: false,
  verbose: false
};

/**
 * @internal
 * @ignore
 */
export async function loadConfig(
  overrides: Partial<MihoOptions> = {}
): Promise<Partial<MihoOptions>> {
  const { config } = await load<Partial<MihoOptions>>({
    name: 'miho',
    cwd: process.cwd(),
    defaultConfig,
    packageJson: true,
    overrides
  });

  return config ?? defaultConfig;
}

export function defineConfig(config: Partial<MihoOptions> = {}) {
  return config;
}
