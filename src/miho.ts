import { glob } from 'glob';
import { MihoPackage, FileData } from './files';
import { defaultConfig } from './config';
import { GitCommit } from './git';
import {
  FileType,
  MihoIgnore,
  isNotBlank,
  HookCallbackMap,
  isTemplateArray,
  LogLevel
} from './utils';
import type {
  MihoGetPackagesOptions,
  MihoHooks,
  MihoOptions,
  MihoHookCallback,
  HookCallbackParameters,
  MihoInternalOptions,
  Nullish,
  CommitOptions
} from './types';

export class Miho {
  #config: Partial<MihoInternalOptions> = {};
  #commit: Nullish<GitCommit>;
  readonly #packages = new Map<number, MihoPackage>();
  readonly #updatedPackages = new Map<number, MihoPackage>();
  readonly #hookCallbackMap = new HookCallbackMap();

  constructor(options: Partial<MihoOptions> = {}) {
    this.#resolveMihoOptions(options);
  }

  /**
   * @internal
   * @ignore
   */
  public readonly l = this.#createLogger();

  public readonly beforeAll = this.#createHookRegisterFn('beforeAll');
  public readonly afterAll = this.#createHookRegisterFn('afterAll');
  public readonly beforeEach = this.#createHookRegisterFn('beforeEach');
  public readonly afterEach = this.#createHookRegisterFn('afterEach');

  /** Search for all packages that meet the requirements. */
  public async search(options: Partial<MihoOptions> = {}): Promise<this> {
    this.#resolveMihoOptions(options);

    let { exclude } = this.#config;
    if (!exclude) exclude = defaultConfig.exclude;
    if (!Array.isArray(exclude)) exclude = [exclude];
    exclude = exclude.filter(isNotBlank);

    const files = await glob(this.#resolvePatterns(), {
      withFileTypes: true,
      ignore: [MihoIgnore.GIT, MihoIgnore.NODE_MODULES, ...exclude]
    });

    const result = await Promise.all(
      files.map((pathObj) => {
        return MihoPackage.create(this, pathObj, this.#config);
      })
    );

    let id = 0;
    result.filter(Boolean).forEach((pkg: MihoPackage) => {
      this.#packages.set(++id, pkg);
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
    const pkg = this.#packages.get(id);
    if (pkg) {
      for (const cb of this.#yieldHookCallbacks('beforeEach')) {
        const returnValue = await cb(
          this.#createHookParameters(new FileData(id, pkg))
        );
        if (returnValue === false) return false;
      }

      await pkg.bump();
      this.#packages.delete(id);
      this.#updatedPackages.set(id, pkg);

      for (const cb of this.#yieldHookCallbacks('afterEach')) {
        await cb(this.#createHookParameters(new FileData(id, pkg)));
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
    for (const cb of this.#yieldHookCallbacks('beforeAll')) {
      const returnValue = await cb(this.#createHookParameters(packages));
      if (returnValue === false) return 0;
    }

    const results = await Promise.all(
      Array.from(this.#packages.keys()).map(this.bump.bind(this))
    );

    for (const cb of this.#yieldHookCallbacks('afterAll')) {
      await cb(this.#createHookParameters(packages));
    }

    return results.filter(Boolean).length;
  }

  /**
   * @internal
   * @ignore
   */
  public shouldCommit() {
    return Boolean(this.#commit) && this.#updatedPackages.size > 0;
  }

  public async commit() {
    if (!this.#commit) {
      throw new Error('Cannot commit: options not set.');
    } else if (this.#updatedPackages.size === 0 && !this.#commit.all) {
      throw new Error('Nothing to commit.');
    }

    await this.#commit.commit(Array.from(this.#updatedPackages.values()));
    this.#updatedPackages.clear();
  }

  /**
   * Returns information on the packages found by Miho.
   *
   * @returns Snapshot of the packages at the time they were found. Modifying any property will have no effect on them.
   */
  public getPackages(options?: MihoGetPackagesOptions): FileData[] {
    let packages: FileData[] = Array.from(this.#packages.entries()).map(
      ([id, pkg]) => new FileData(id, pkg)
    );

    if (options?.filter) {
      packages = packages.filter(options.filter);
    }

    return packages;
  }

  /** Find a package by its name among the ones previously found by Miho. */
  public getPackageByName(packageName: string): FileData | null {
    const packages = this.getPackages();
    return packages.find(({ name }) => name === packageName) ?? null;
  }

  /** Register multiple hooks simultaneously. */
  public resolveHooks<T extends keyof MihoHooks>(
    hooks: Partial<MihoHooks>
  ): this {
    Object.entries(hooks).forEach(([key, value]: [T, MihoHooks[T]]) => {
      this.#hookCallbackMap.set(key, value);
    });

    return this;
  }

  #resolveMihoOptions(options: Partial<MihoOptions>) {
    const { hooks, commit, ...config } = options;
    this.#config = { ...this.#config, ...config };
    if (commit) this.#resolveCommitOptions(commit);
    if (hooks) this.resolveHooks(hooks);
  }

  #resolveCommitOptions(options: Partial<CommitOptions>) {
    if (typeof options.message === 'string' || options.all === true) {
      this.#commit = new GitCommit({
        ...(this.#commit ? { ...this.#commit } : {}),
        ...options
      });
    } else {
      this.#commit = null;
    }
  }

  #resolvePatterns() {
    if (!this.#config.recursive) return FileType.PACKAGE_JSON;
    let patterns = this.#config.include ?? [];
    if (typeof patterns === 'string') patterns = [patterns];

    patterns = patterns.map((i) => i.trim());
    patterns = patterns.filter((i) => i.length > 0);
    if (patterns.length === 0) return defaultConfig.include;
    return patterns;
  }

  #createHookRegisterFn<T extends keyof MihoHooks>(hookName: T) {
    return (cb: MihoHooks[T]) => {
      this.#hookCallbackMap.set(hookName, cb);
      return this;
    };
  }

  #createHookParameters<T>(data: T): HookCallbackParameters<T> {
    return {
      miho: this,
      data
    };
  }

  *#yieldHookCallbacks<T extends keyof MihoHooks>(hookName: T) {
    const cbs = this.#hookCallbackMap.get(hookName) as MihoHookCallback<T>[];
    for (const cb of cbs) yield cb;
  }

  #createLogger(logLevel: LogLevel = LogLevel.HIGH) {
    const self = this;
    function l(options: LogLevel): typeof l;
    function l(strings: TemplateStringsArray, ...subs: unknown[]): void;
    function l(raw: TemplateStringsArray | LogLevel, ...subs: unknown[]) {
      if (isTemplateArray(raw)) {
        const result = String.raw({ raw }, ...subs);
        const log = () => console.log(result);

        if (!self.#config.silent) {
          switch (logLevel) {
            case LogLevel.LOW: {
              if (self.#config.verbose) log();
              return;
            }
            case LogLevel.NORMAL: {
              log();
              return;
            }
          }
        }

        if (logLevel === LogLevel.HIGH) log();
        return;
      } else {
        return self.#createLogger(raw);
      }
    }

    return l.bind(this);
  }
}
