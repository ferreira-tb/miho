import { glob } from 'glob';
import { MihoPackage } from './package';
import { defaultConfig } from './config';
import { Filename, MihoIgnore, isNotBlankString } from './utils';
import type { GetPackagesOptions, MihoOptions, PackageData } from './types';

export class Miho {
        private readonly packages = new Map<number, MihoPackage>();

     private constructor(packages: MihoPackage[]) {
    let id = 0;
    packages.forEach((pkg) => {
      this.packages.set(++id, pkg);
    });
  }

  /**
   * Bump a single package.
   *
   * You can get the id of the packages using the {@link Miho.getPackages} method.
   */
  public async bump(id: number): Promise<Miho> {
    const pkg = this.packages.get(id);
    if (pkg) {
      await pkg.bump();
      this.packages.delete(id);
    }

    return this;
  }

  /** Bumps all packages found by Miho. */
  public async bumpAll(): Promise<Miho> {
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

  /** Search for all packages that meet the requirements. */
  public static async init(config: Partial<MihoOptions> = {}): Promise<Miho> {
    let { exclude } = config;
    if (!Array.isArray(exclude)) exclude = defaultConfig.exclude;
    exclude = exclude.filter(isNotBlankString);

    const files = await glob(this.resolvePatterns(config), {
      withFileTypes: true,
      ignore: [MihoIgnore.GIT, MihoIgnore.NODE_MODULES, ...exclude]
    });

    const result = await Promise.all(
      files.map((filePath) => {
        return MihoPackage.create(filePath, config);
      })
    );

    return new Miho(result.filter(Boolean) as MihoPackage[]);
  }

  private static resolvePatterns(config: Partial<MihoOptions>) {
    if (!config.recursive) return Filename.PACKAGE_JSON;
    let patterns = config.include ?? [];
    if (typeof patterns === 'string') patterns = [patterns];

    patterns = patterns.map((i) => i.trim());
    patterns = patterns.filter((i) => i.length > 0);
    if (patterns.length === 0) return defaultConfig.include;
    return patterns;
  }
}
