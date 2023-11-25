import { defineConfig, type ExternalOption } from 'rollup';
import typescript from '@rollup/plugin-typescript';
import dts from 'vite-plugin-dts';

const externalDeps: ExternalOption = [
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
