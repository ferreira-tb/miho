import * as fs from 'node:fs/promises';
import detectIndent from 'detect-indent';
import semver, { type ReleaseType } from 'semver';
import { defaultConfig } from './config';
import type { Path } from 'glob';
import type { MihoOptions } from '../types';

type MihoPackageConstructor = {
  readonly raw: Path;
  readonly packageName: string | null;
  readonly version: string;
  readonly indent: string;
};

/** @internal */
export class MihoPackage {
  private readonly fullpath: string;
  private readonly _packageName: string | null;
  private readonly _version: string;
  private readonly indent: string;
  private release: MihoOptions['release'];
  private preid: MihoOptions['preid'];
  private _newVersion: string | null = null;

  private constructor(
    config: Partial<MihoOptions>,
    options: MihoPackageConstructor
  ) {
    const name = options.packageName;

    this.fullpath = options.raw.fullpath();
    this._packageName = name;
    this._version = options.version;
    this.indent = options.indent;

    this.release = config.release ?? defaultConfig.release;
    this.preid = config.preid ?? defaultConfig.preid;

    const override = name ? config.overrides?.[name] : null;
    if (override) {
      switch (typeof override) {
        case 'object': {
          this.release = override.release ?? defaultConfig.release;
          this.preid = override.preid ?? defaultConfig.preid;
          break;
        }
        case 'string':
        case 'number':
          this.release = override;
      }
    }

    if (typeof this.release === 'number') {
      this._newVersion = semver.coerce(this.release)?.raw ?? null;
    } else if (MihoPackage.isReleaseType(this.release)) {
      if (this.release.startsWith('pre')) {
        this._newVersion = semver.inc(this._version, this.release, this.preid);
      } else {
        this._newVersion = semver.inc(this._version, this.release);
      }
    } else {
      this._newVersion = semver.clean(this.release);
    }
  }

  public async bump() {
    if (typeof this._newVersion !== 'string') {
      throw new TypeError(`Invalid version: ${this._newVersion}`);
    }

    const file = await fs.readFile(this.fullpath, 'utf-8');
    const pkg = JSON.parse(file) as Record<string, unknown>;
    pkg.version = this._newVersion;

    const jsonString = JSON.stringify(pkg, null, this.indent);
    await fs.writeFile(this.fullpath, jsonString, 'utf-8');
  }

  get packageName() {
    return this._packageName;
  }

  get version() {
    return this._version;
  }

  get newVersion() {
    return this._newVersion;
  }

  public static async create(raw: Path, config: Partial<MihoOptions> = {}) {
    if (!raw.isFile()) return null;

    const file = await fs.readFile(raw.fullpath(), 'utf-8');
    const pkg = JSON.parse(file) as Record<string, unknown>;

    const packageName = typeof pkg.name === 'string' ? pkg.name : null;
    if (packageName && Array.isArray(config.ignore)) {
      for (const rule of config.ignore) {
        if (
          (typeof rule === 'string' && rule === packageName) ||
          (rule instanceof RegExp && rule.test(packageName))
        ) {
          return null;
        }
      }
    }

    const version = semver.clean(String(pkg.version));
    if (!version) return null;

    return new this(config, {
      raw,
      packageName,
      version,
      indent: detectIndent(file).indent
    });
  }

  private static isReleaseType(value: unknown): value is ReleaseType {
    if (typeof value !== 'string') return false;
    return semver.RELEASE_TYPES.some((r) => r === value);
  }
}
