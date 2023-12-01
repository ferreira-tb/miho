import dts from 'vite-plugin-dts';
import { defineConfig } from 'rollup';
import typescript from '@rollup/plugin-typescript';

const externalDeps = [
  /^node:/,
  /^yargs/,
  'c12',
  'chalk',
  'detect-indent',
  'execa',
  'glob',
  'prompts',
  'semver'
];

/**
 * @see https://github.com/qmhc/vite-plugin-dts#internal-error-occurs-when-using-rolluptypes-true
 * @see https://github.com/microsoft/rushstack/issues/3875#issuecomment-1746164303
 */
export default defineConfig([
  {
    plugins: [typescript(), dts({ rollupTypes: true })],
    external: externalDeps,
    input: 'src/index.ts',
    output: [
      {
        file: 'dist/index.mjs',
        format: 'es',
        generatedCode: 'es2015'
      },
      {
        file: 'dist/index.cjs',
        format: 'cjs',
        generatedCode: 'es2015'
      }
    ]
  },
  {
    plugins: [typescript()],
    external: externalDeps,
    input: 'src/cli/index.ts',
    output: [
      {
        file: 'dist/cli.mjs',
        format: 'es',
        generatedCode: 'es2015'
      }
    ]
  }
]);
