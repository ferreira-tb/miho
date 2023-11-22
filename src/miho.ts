import { glob } from 'glob';
import { MihoPackage } from './package';
import { defaultConfig } from './config';
import type { GetPackagesOptions, MihoOptions, PackageData } from '../types';

export class Miho {
  private readonly packages = new Map<number, MihoPackage>();

  private constructor(packages: MihoPackage[]) {
    let id = 0;
    packages.forEach((pkg) => {
      this.packages.set(++id, pkg);
    });
  }

  public async bump(id: number): Promise<Miho> {
    const pkg = this.packages.get(id);
    if (pkg) {
      await pkg.bump();
      this.packages.delete(id);
    }

    return this;
  }

  public async bumpAll(): Promise<Miho> {
    await Promise.all(
      Array.from(this.packages.keys()).map(this.bump.bind(this))
    );

    return this;
  }

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

  public static async init(config: Partial<MihoOptions> = {}): Promise<Miho> {
    let exclude = Array.isArray(config.exclude)
      ? config.exclude
      : defaultConfig.exclude;
    exclude = exclude.filter((e) => typeof e === 'string' && e.length > 0);

    const pattern = `${config.recursive ? '**/' : ''}package.json`;
    const files = await glob(pattern, {
      withFileTypes: true,
      ignore: ['.git/**', 'node_modules/**', ...exclude]
    });

    const result = await Promise.all(
      files.map((filePath) => {
        return MihoPackage.create(filePath, config);
      })
    );

    return new Miho(result.filter(Boolean) as MihoPackage[]);
  }
}
