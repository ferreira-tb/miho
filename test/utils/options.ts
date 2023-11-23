import { MihoMock } from './mock';
import type { MihoOptions } from '../../src';

export const defaultOptions: Partial<MihoOptions> = {
  include: [MihoMock.TEMP_GLOB.toString()],
  filter: [/miho/],
  recursive: true
};
