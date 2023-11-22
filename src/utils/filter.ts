export function isNotBlankString(value: unknown) {
  return typeof value === 'string' && value.length > 0;
}
