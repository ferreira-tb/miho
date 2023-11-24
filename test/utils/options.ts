import type { MihoOptions } from '../../src';

export function getDefaultOptions(testName: string): Partial<MihoOptions> {
  return {
    release: 'patch',
    include: [`.temp/${testName}/**`],
    filter: [/miho/],
    recursive: true,
    verbose: false
  };
}
