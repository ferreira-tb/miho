import process from 'node:process';
import { loadConfig as load } from 'c12';
import type { MihoOptions, Nullish } from './types';

/**
 * @internal
 * @ignore
 */
export const defaultConfig: MihoOptions = {
  exclude: [],
  filter: [],
  include: '**/',
  overrides: {},
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
export async function loadConfig(overrides: Partial<MihoOptions> = {}) {
  const { config } = await load<Partial<MihoOptions>>({
    name: 'miho',
    cwd: process.cwd(),
    defaultConfig,
    packageJson: true,
    overrides
  });

  return (config as Nullish<MihoOptions>) ?? defaultConfig;
}

export function defineConfig(config: Partial<MihoOptions> = {}) {
  return config;
}
