export * from './enum';

export function isNotBlank(value: unknown): value is string {
  return typeof value === 'string' && value.length > 0;
}

/**
 * @internal
 * @ignore
 */
export function isTemplateArray(value: unknown): value is TemplateStringsArray {
  return Array.isArray(value);
}
