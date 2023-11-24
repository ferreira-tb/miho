import { beforeEach, describe, expect, it } from 'vitest';
import { createMockPackages, toHaveBeenBumped } from './utils';

const testName = 'cli';
beforeEach(() => createMockPackages(testName));

expect.extend({
  toHaveBeenBumped: toHaveBeenBumped(testName, this)
});

describe('ab', () => {
  it('a', () => {});
});
