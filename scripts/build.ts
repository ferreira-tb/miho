import * as path from 'node:path';
import * as fs from 'node:fs/promises';
import process from 'node:process';
import { fileURLToPath } from 'node:url';
import { existsSync as exists } from 'node:fs';
import { execa } from 'execa';

try {
  const dirname = path.dirname(fileURLToPath(import.meta.url));
  const dist = path.resolve(dirname, '../dist');
  if (exists(dist)) await fs.rm(dist, { recursive: true });
  await execa('run-s', ['rollup', 'minify'], { stdio: 'inherit' });
} catch (err) {
  console.error(err);
  process.exit(1);
}
