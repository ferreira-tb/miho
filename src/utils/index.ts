export * from './enum';
export * from './log';
export * from '../hooks/map';

export function isNotBlank(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0;
}
