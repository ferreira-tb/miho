import { glob } from 'glob';
import { MihoPackage, PackageData } from './files/package';
import { defaultConfig } from './config';
import {
  Filename,
  MihoIgnore,
  isNotBlankString,
  HookCallbackMap
} from './utils';
import type {
  GetPackagesOptions,
  MihoHooks,
  MihoOptions,
  MihoHookCallback,
  CliOptions
} from './types';

export class Miho {
  private config: Partial<CliOptions> = {};
  private readonly packages = new Map<number, MihoPackage>();
  private readonly hookCallbackMap = new HookCallbackMap();

  public readonly beforeAll = this.createHookRegisterFn('beforeAll');
  public readonly afterAll = this.createHookRegisterFn('afterAll');
  public readonly beforeEach = this.createHookRegisterFn('beforeEach');
  public readonly afterEach = this.createHookRegisterFn('afterEach');

  constructor(options: Partial<MihoOptions> = {}) {
    this.resolveMihoOptions(options);
  }

  /** Search for all packages that meet the requirements. */
  public async search(options: Partial<MihoOptions> = {}): Promise<this> {
    this.resolveMihoOptions(options);

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
   * @returns Whether the package was successfully bumped.
   */
  public async bump(id: number): Promise<boolean> {
    const pkg = this.packages.get(id);
    if (pkg) {
      for (const cb of this.yieldHookCallbacks('beforeEach')) {
        const returnValue = await cb(new PackageData(id, pkg));
        if (returnValue === false) return false;
      }

      await pkg.bump();
      this.packages.delete(id);

      for (const cb of this.yieldHookCallbacks('afterEach')) {
        await cb(new PackageData(id, pkg));
      }
    }

    return true;
  }

  /**
   * Bumps all packages found by Miho.
   * @returns Number of packages successfully bumped.
   */
  public async bumpAll(): Promise<number> {
    const packages = this.getPackages();
    for (const cb of this.yieldHookCallbacks('beforeAll')) {
      const returnValue = await cb(packages);
      if (returnValue === false) return 0;
    }

    const results = await Promise.all(
      Array.from(this.packages.keys()).map(this.bump.bind(this))
    );

    for (const cb of this.yieldHookCallbacks('afterAll')) {
      await cb(packages);
    }

    return results.filter(Boolean).length;
  }

  /**
   * Returns information on the packages found by Miho.
   *
   * The objects returned by this method are just a snapshot of
   * the state of the packages at the time they were found.
   */
  public getPackages(options?: GetPackagesOptions): PackageData[] {
    let packages: PackageData[] = Array.from(this.packages.entries()).map(
      ([id, pkg]) => new PackageData(id, pkg)
    );

    if (options?.filter) {
      packages = packages.filter(options.filter);
    }

    return packages;
  }

  /** Register multiple hooks simultaneously. */
  public resolveHooks<T extends keyof MihoHooks>(
    hooks: Partial<MihoHooks>
  ): this {
    Object.entries(hooks).forEach(([key, value]: [T, MihoHooks[T]]) => {
      this.hookCallbackMap.set(key, value);
    });

    return this;
  }

  private resolveMihoOptions(options: Partial<MihoOptions>) {
    const { hooks, ...config } = options;
    this.config = { ...this.config, ...config };
    if (hooks) this.resolveHooks(hooks);
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

  private createHookRegisterFn<T extends keyof MihoHooks>(hookName: T) {
    return (cb: MihoHooks[T]) => {
      this.hookCallbackMap.set(hookName, cb);
      return this;
    };
  }

  private *yieldHookCallbacks<T extends keyof MihoHooks>(hookName: T) {
    const cbs = this.hookCallbackMap.get(hookName) as MihoHookCallback<T>[];
    for (const cb of cbs) yield cb;
  }
}
