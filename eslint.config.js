import config from '@tb-dev/eslint-config';

export default config({
  project: [
    'tsconfig.json',
    'tsconfig.miho.json',
    'docs/tsconfig.json',
    'scripts/tsconfig.json',
    'test/tsconfig.json'
  ],
  overrides: {
    typescript: {
      '@typescript-eslint/no-confusing-void-expression': 'off'
    }
  }
});
