import { execa } from 'execa';
import { beforeEach, expect, it } from 'vitest';
import { Miho } from '../src';
import {
  createMockPackages,
  toHaveBeenBumped,
  getDefaultOptions
} from './utils';

const testName = 'cli';
beforeEach(() => createMockPackages(testName));

expect.extend({
  toHaveBeenBumped: toHaveBeenBumped(testName, this)
});

const options = getDefaultOptions(testName);
if (!Array.isArray(options.include)) {
  throw new Error(`Invalid "include" option.`);
}

const include = `-i ${options.include.join(' ')}`;

it('should bump', async () => {
  const miho = await new Miho(options).search();
  const pkgs = miho.getPackages();

  await execa('npx', ['miho', 'patch', '-r', `${include}`, '--no-ask']);
  await expect(pkgs).toHaveBeenBumped();
});
