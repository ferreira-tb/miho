import { glob } from 'glob';
import type { Options as ExecaOptions } from 'execa';
import { MihoPackage, FileData } from './files';
import { defaultConfig } from './config';
import { GitCommit } from './git';
import { MihoEmitter, MihoEvent } from './hooks';
import {
  FileType,
  MihoIgnore,
  isNotBlank,
  isTemplateArray,
  LogLevel
} from './utils';
import type {
  MihoGetPackagesOptions,
  MihoOptions,
  MihoInternalOptions,
  CommitOptions
} from './types';

export class Miho extends MihoEmitter {
  #config: Partial<MihoInternalOptions> = {};
  #commit: GitCommit = new GitCommit();
  readonly #packages = new Map<number, MihoPackage>();
  readonly #updatedPackages = new Map<number, MihoPackage>();

  constructor(options: Partial<MihoOptions> = {}) {
    super();
    this.#resolveMihoOptions(options);
  }

  /**
   * @internal
   * @ignore
   */
  public readonly l = this.#createLogger();

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
      const defaultPrevented = await this.executeHook(
        new MihoEvent('beforeEach', {
          miho: this,
          data: new FileData(id, pkg),
          cancelable: true
        })
      );
      if (defaultPrevented) return false;

      await pkg.bump();
      this.#packages.delete(id);
      this.#updatedPackages.set(id, pkg);

      await this.executeHook(
        new MihoEvent('afterEach', {
          miho: this,
          data: new FileData(id, pkg)
        })
      );
    }

    return true;
  }

  /**
   * Bumps all packages found by Miho.
   * @returns Number of packages successfully bumped.
   */
  public async bumpAll(): Promise<number> {
    const packages = this.getPackages();
    const defaultPrevented = await this.executeHook(
      new MihoEvent('beforeAll', {
        miho: this,
        data: packages,
        cancelable: true
      })
    );
    if (defaultPrevented) return 0;

    const results = await Promise.all(
      Array.from(this.#packages.keys()).map(this.bump.bind(this))
    );

    await this.executeHook(
      new MihoEvent('afterAll', {
        miho: this,
        data: packages
      })
    );

    return results.filter(Boolean).length;
  }

  /** Commit the modified packages. */
  public async commit(options: Partial<CommitOptions> = {}): Promise<void> {
    this.#resolveCommitOptions(options);

    if (this.#updatedPackages.size === 0 && !this.#commit.all) {
      throw new Error('Nothing to commit.');
    }

    const execaOptions: ExecaOptions = this.#config.verbose
      ? { stdout: 'inherit' }
      : {};

    const entries = Array.from(this.#updatedPackages.entries());
    const data = entries.map(([id, pkg]) => new FileData(id, pkg));

    const defaultPrevented = await this.executeHook(
      new MihoEvent('beforeCommit', { miho: this, data, cancelable: true })
    );
    if (defaultPrevented) return;

    const packages = entries.map(([, pkg]) => pkg);
    await this.#commit.commit(packages, execaOptions);
    this.#updatedPackages.clear();

    await this.executeHook(new MihoEvent('afterCommit', { miho: this, data }));

    if (this.#commit.push) {
      const defaultPrevented = await this.executeHook(
        new MihoEvent('beforePush', { miho: this, data, cancelable: true })
      );
      if (defaultPrevented) return;

      await this.#commit.pushCommit(execaOptions);

      await this.executeHook(new MihoEvent('afterPush', { miho: this, data }));
    }
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

  #resolveMihoOptions(options: Partial<MihoOptions>) {
    const { hooks, commit, ...config } = options;
    this.#config = { ...this.#config, ...config };
    if (commit) this.#resolveCommitOptions(commit);
    if (hooks) this.resolveListeners(hooks);
  }

  #resolveCommitOptions(options: Partial<CommitOptions>) {
    this.#commit = new GitCommit({
      ...this.#commit,
      ...options
    });
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
