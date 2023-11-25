/**
 * @internal
 * @ignore
 */
export const enum LogLevel {
  /** Only displayed if `--verbose` flag is set. */
  LOW = 0,
  /** Not so important. Can be omitted if `--silent`. */
  NORMAL = 1,
  /** Important log. Should always be displayed. */
  HIGH = 2
}

/**
 * @internal
 * @ignore
 */
export function isTemplateArray(value: unknown): value is TemplateStringsArray {
  return Array.isArray(value);
}
