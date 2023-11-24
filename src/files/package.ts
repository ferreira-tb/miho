import process from 'node:process';
import path from 'node:path';
import fs from 'node:fs/promises';
import detectIndent from 'detect-indent';
import semver, { type ReleaseType } from 'semver';
import { defaultConfig } from '../config';
import { LogLevel } from '../utils';
import type { Path } from 'glob';
import type { MihoOptions } from '../types';
import type { Miho } from '../miho';

type MihoPackageConstructor = {
  readonly raw: Path;
  readonly packageName: string | null;
  readonly version: string;
  readonly indent: string;
};

/**
 * @internal
 * @ignore
 */
export class MihoPackage {
  readonly #fullpath: string;
  readonly #packageName: string | null;
  readonly #version: string;
  readonly #indent: string;
  #release: MihoOptions['release'];
  #preid: MihoOptions['preid'];
  #newVersion: string | null = null;

  private constructor(
    config: Partial<MihoOptions>,
    options: MihoPackageConstructor
  ) {
    const name = options.packageName;

    this.#fullpath = options.raw.fullpath();
    this.#packageName = name;
    this.#version = options.version;
    this.#indent = options.indent;

    this.#release = config.release ?? defaultConfig.release;
    this.#preid = config.preid ?? defaultConfig.preid;

    const override = name ? config.overrides?.[name] : null;
    if (override) {
      switch (typeof override) {
        case 'string':
        case 'number':
          this.#release = override;
          break;
        case 'object': {
          this.#release = override.release ?? defaultConfig.release;
          this.#preid = override.preid ?? defaultConfig.preid;
        }
      }
    }

    if (typeof this.#release === 'number') {
      this.#newVersion = semver.coerce(this.#release)?.raw ?? null;
    } else if (MihoPackage.#isReleaseType(this.#release)) {
      if (this.#release.startsWith('pre')) {
        this.#newVersion = semver.inc(
          this.#version,
          this.#release,
          this.#preid
        );
      } else {
        this.#newVersion = semver.inc(this.#version, this.#release);
      }
    } else {
      this.#newVersion = semver.clean(this.#release);
    }
  }

  public async bump() {
    if (typeof this.#newVersion !== 'string') {
      throw new TypeError(`Invalid version: ${this.#newVersion}`);
    }

    const file = await fs.readFile(this.#fullpath, 'utf-8');
    const pkg = JSON.parse(file) as Record<string, unknown>;
    pkg.version = this.#newVersion;

    const jsonString = JSON.stringify(pkg, null, this.#indent);
    await fs.writeFile(this.#fullpath, jsonString, 'utf-8');
  }

  get packageName() {
    return this.#packageName;
  }

  get version() {
    return this.#version;
  }

  get newVersion() {
    return this.#newVersion;
  }

  public static async create(
    miho: Miho,
    raw: Path,
    config: Partial<MihoOptions> = {}
  ) {
    if (!raw.isFile()) return null;

    const fullpath = raw.fullpath();
    const file = await fs.readFile(fullpath, 'utf-8');
    const pkg = JSON.parse(file) as Record<string, unknown>;

    const packageName = typeof pkg.name === 'string' ? pkg.name : null;
    if (packageName && Array.isArray(config.filter)) {
      for (const rule of config.filter) {
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

    const mihoPackage = new MihoPackage(config, {
      raw,
      packageName,
      version,
      indent: detectIndent(file).indent
    });

    if (config.verbose) {
      const relative = path.relative(process.cwd(), fullpath);
      miho.l(LogLevel.LOW)`Found: ${relative}`;
    }

    return mihoPackage;
  }

  static #isReleaseType(value: unknown): value is ReleaseType {
    if (typeof value !== 'string') return false;
    return semver.RELEASE_TYPES.some((r) => r === value);
  }
}

export class PackageData {
  readonly id: number;
  readonly name: string | null;
  readonly version: string;
  readonly newVersion: string | null;

  constructor(id: number, pkg: MihoPackage) {
    this.id = id;
    this.name = pkg.packageName;
    this.version = pkg.version;
    this.newVersion = pkg.newVersion;
  }
}
