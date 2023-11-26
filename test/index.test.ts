import process from 'node:process';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { detectPackageManager, PackageManager } from '../src';

describe('detectPackageManager', () => {
  const mockDir = path.join(process.cwd(), 'test');

  it.concurrent('should detect pnpm', async () => {
    const pm = await detectPackageManager();
    expect(pm).toBe(PackageManager.PNPM);
  });

  it.concurrent('should default to npm', async () => {
    const pm = await detectPackageManager({ cwd: mockDir });
    expect(pm).toBe(PackageManager.NPM);
  });

  it.concurrent('should default to yarn', async () => {
    const pm = await detectPackageManager({
      cwd: mockDir,
      default: PackageManager.YARN
    });
    expect(pm).toBe(PackageManager.YARN);
  });
});
