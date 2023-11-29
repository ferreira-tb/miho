import * as path from 'node:path';
import process from 'node:process';
import * as fs from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { existsSync as exists } from 'node:fs';
import { $ } from 'execa';

try {
  const dirname = path.dirname(fileURLToPath(import.meta.url));
  const dist = path.resolve(dirname, '../dist');
  if (exists(dist)) await fs.rm(dist, { recursive: true });
  await $({ stdio: 'inherit' })`run-s rollup minify`;
} catch (err) {
  console.error(err);
  process.exit(1);
}
