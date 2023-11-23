import { glob } from 'glob';
import { MihoPackage } from './package';
import { defaultConfig } from './config';
import { Filename, MihoIgnore, isNotBlankString } from './utils';
import type { GetPackagesOptions, MihoOptions, PackageData } from './types';

export class Miho {
  private readonly packages = new Map<number, MihoPackage>();

  constructor(private readonly config: Partial<MihoOptions> = {}) {}

  /** Search for all packages that meet the requirements. */
  public async search(): Promise<this> {
    let { exclude } = this.config;
    if (!Array.isArray(exclude)) exclude = defaultConfig.exclude;
    exclude = exclude.filter(isNotBlankString);

    const files = await glob(this.resolvePatterns(), {
      withFileTypes: true,
      ignore: [MihoIgnore.GIT, MihoIgnore.NODE_MODULES, ...exclude]
    });

    const result = await Promise.all(
      files.map((filePath) => {
        return MihoPackage.create(filePath, this.config);
      })
    );

    let id = 0;
    result.filter(Boolean).forEach((pkg: MihoPackage) => {
      this.packages.set(++id, pkg);
    });

    return this;
  }

  /**
   * Bump a single package.
   *
   * You can get the id of the packages using the {@link getPackages} method.
   */
  public async bump(id: number): Promise<this> {
    const pkg = this.packages.get(id);
    if (pkg) {
      await pkg.bump();
      this.packages.delete(id);
    }

    return this;
  }

  /** Bumps all packages found by Miho. */
  public async bumpAll(): Promise<this> {
    await Promise.all(
      Array.from(this.packages.keys()).map(this.bump.bind(this))
    );

    return this;
  }

  /**
   * Returns information on the packages found by Miho.
   *
   * The objects returned by this method are just a snapshot of
   * the state of the packages at the time they were found.
   * @param options
   */
  public getPackages(options?: GetPackagesOptions): PackageData[] {
    let packages: PackageData[] = Array.from(this.packages.entries()).map(
      ([id, pkg]) => {
        return {
          id,
          name: pkg.packageName,
          version: pkg.version,
          newVersion: pkg.newVersion
        };
      }
    );

    if (options?.filter) {
      packages = packages.filter(options.filter);
    }

    return packages;
  }

  private resolvePatterns() {
    if (!this.config.recursive) return Filename.PACKAGE_JSON;
    let patterns = this.config.include ?? [];
    if (typeof patterns === 'string') patterns = [patterns];

    patterns = patterns.map((i) => i.trim());
    patterns = patterns.filter((i) => i.length > 0);
    if (patterns.length === 0) return defaultConfig.include;
    return patterns;
  }
}
