import process from 'node:process';
import { loadConfig } from 'c12';
import type { MihoOptions } from '../types';

/** @ignore */
export const defaultConfig: MihoOptions = {
  preid: 'alpha',
  release: 'patch',
  recursive: false,
  ignore: [],
  exclude: []
};

/** @internal */
export async function loadMihoConfig(overrides: Partial<MihoOptions> = {}) {
  const { config } = await loadConfig<MihoOptions>({
    name: 'miho',
    cwd: process.cwd(),
    defaultConfig,
    packageJson: true,
    overrides: { ...(overrides as MihoOptions) }
  });

  return config ?? defaultConfig;
}

export function defineConfig(config: Partial<MihoOptions> = {}) {
  return config;
}
