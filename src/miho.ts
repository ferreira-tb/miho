import process from 'node:process';
import fs from 'node:fs/promises';
import { glob, type Path } from 'glob';
import { execa, type Options as ExecaOptions } from 'execa';
import { MihoPackage, FileData } from './files';
import { defaultConfig } from './config';
import { MihoEmitter, MihoEvent } from './hooks';
import { GitCommit } from './git';
import { createJobSkipChecker } from './jobs';
import {
  detectPackageManager,
  isPackageManager
} from './utils/package-manager';
import {
  FileType,
  MihoIgnore,
  MihoJob,
  isNotBlank,
  isTemplateArray,
  LogLevel,
  PackageManager
} from './utils';
import type {
  MihoGetPackagesOptions,
  MihoOptions,
  MihoInternalOptions,
  CommitOptions,
  MihoCommitArgs,
  JobFunction,
  JobOptions,
  JobFunctionOptions
} from './types';

export class Miho extends MihoEmitter {
  #id = 0;
  #gitCommit: GitCommit = new GitCommit();
  #jobs: Partial<JobOptions> = {};
  #config: Partial<MihoInternalOptions> = {};
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

    const patterns = await this.#resolvePatterns();
    const files = await glob(patterns, {
      withFileTypes: true,
      ignore: [MihoIgnore.GIT, MihoIgnore.NODE_MODULES, ...exclude]
    });

    const packages = await this.#createPackages(files);
    packages.forEach((pkg) => {
      this.#packages.set(++this.#id, pkg);
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
    const shouldSkip = createJobSkipChecker(this.#jobs);
    if (shouldSkip(MihoJob.BUMP)) return false;

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
    const shouldSkip = createJobSkipChecker(this.#jobs);
    if (shouldSkip(MihoJob.BUMP)) return 0;

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

  /** Build the project. */
  public async build(options: JobFunctionOptions = {}) {
    const shouldSkip = createJobSkipChecker(this.#jobs);
    if (shouldSkip(MihoJob.BUILD)) return;

    await this.#setPackageManager(options.cwd);
    const pm = this.#config.packageManager!;

    if (this.#jobs.build === true) {
      const execaOptions = this.#createExecaOptions(options.cwd);
      await execa(pm, ['run', MihoJob.BUILD], execaOptions);
    } else if (typeof this.#jobs.build === 'function') {
      await this.#jobs.build({
        name: MihoJob.BUILD,
        miho: this,
        cwd: options.cwd ?? process.cwd()
      });
    }
  }

  /** Run tests. */
  public async test(options: JobFunctionOptions = {}) {
    const shouldSkip = createJobSkipChecker(this.#jobs);
    if (shouldSkip(MihoJob.TEST)) return;

    await this.#setPackageManager(options.cwd);
    const pm = this.#config.packageManager!;

    if (this.#jobs.test === true) {
      const execaOptions = this.#createExecaOptions(options.cwd);
      await execa(pm, ['run', MihoJob.TEST], execaOptions);
    } else if (typeof this.#jobs.test === 'function') {
      await this.#jobs.test({
        name: MihoJob.TEST,
        miho: this,
        cwd: options.cwd ?? process.cwd()
      });
    }
  }

  /** Commit the modified packages. */
  public async commit(args: Partial<MihoCommitArgs> = {}): Promise<void> {
    const { dryRun, ...options } = args;
    const shouldSkip = createJobSkipChecker(this.#jobs);
    if (shouldSkip(MihoJob.COMMIT)) return;

    this.#resolveCommitOptions(options);

    if (this.#updatedPackages.size === 0 && !this.#gitCommit.all) {
      throw new Error('Nothing to commit.');
    }

    const execaOptions = this.#createExecaOptions();

    const entries = Array.from(this.#updatedPackages.entries());
    const data = entries.map(([id, pkg]) => new FileData(id, pkg));

    const defaultPrevented = await this.executeHook(
      new MihoEvent('beforeCommit', { miho: this, data, cancelable: true })
    );
    if (defaultPrevented) return;

    await this.#gitCommit.commit({
      miho: this,
      packages: entries.map(([, pkg]) => pkg),
      execaOptions,
      dryRun
    });

    this.#updatedPackages.clear();

    await this.executeHook(new MihoEvent('afterCommit', { miho: this, data }));

    if (this.#gitCommit.push) {
      const defaultPrevented = await this.executeHook(
        new MihoEvent('beforePush', { miho: this, data, cancelable: true })
      );
      if (defaultPrevented) return;

      await this.#gitCommit.pushCommit({ miho: this, execaOptions, dryRun });

      await this.executeHook(new MihoEvent('afterPush', { miho: this, data }));
    }
  }

  /** Publish the package. */
  public async publish(options: JobFunctionOptions = {}) {
    const shouldSkip = createJobSkipChecker(this.#jobs);
    if (shouldSkip(MihoJob.PUBLISH)) return;

    await this.#setPackageManager(options.cwd);
    const pm = this.#config.packageManager!;

    if (this.#jobs.publish === true) {
      const execaOptions = this.#createExecaOptions(options.cwd);
      const args: string[] = [MihoJob.PUBLISH];
      if (pm === PackageManager.YARN) args.unshift(PackageManager.NPM);
      await execa(pm, args, execaOptions);
    } else if (typeof this.#jobs.publish === 'function') {
      await this.#jobs.publish({
        name: MihoJob.PUBLISH,
        miho: this,
        cwd: options.cwd ?? process.cwd()
      });
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
  public getPackageByName(packageName: string | RegExp): FileData | null {
    const packages = this.getPackages();
    const pkg = packages.find(({ name }) => {
      if (typeof packageName === 'string') return name === packageName;
      return packageName.test(name ?? '');
    });

    return pkg ?? null;
  }

  public setJob<T extends keyof JobFunction>(job: T, value: JobFunction[T]) {
    this.#resolveJobOptions({ [job]: value });
  }

  #resolveMihoOptions(options: Partial<MihoOptions>) {
    const { hooks, commit, jobs, ...config } = options;
    this.#config = { ...this.#config, ...config };
    if (commit) this.#resolveCommitOptions(commit);
    if (hooks) this.resolveListeners(hooks);
    if (jobs) this.#resolveJobOptions(jobs);
  }

  #resolveCommitOptions(options: Partial<CommitOptions>) {
    this.#gitCommit = new GitCommit({
      ...this.#gitCommit,
      ...options
    });
  }

  #resolveJobOptions(jobs: Partial<JobOptions>) {
    this.#jobs = { ...this.#jobs, ...jobs };
  }

  async #resolvePatterns(): Promise<string[]> {
    let patterns: string[] = [FileType.PACKAGE_JSON];
    if (!this.#config.recursive) return patterns;

    if (this.#config.include) {
      const include = Array.isArray(this.#config.include)
        ? this.#config.include
        : [this.#config.include];

      patterns.push(...include);
    }

    patterns = patterns.map((i) => i.trim());
    patterns = patterns.filter((i) => i.length > 0);

    // If only "FileType.PACKAGE_JSON" is inside the array.
    if (patterns.length === 1) {
      let dirents = await fs.readdir(process.cwd(), { withFileTypes: true });
      dirents = dirents.filter((ent) => ent.isDirectory());

      patterns.push(
        ...dirents.map((ent) => `${ent.name}/**/${FileType.PACKAGE_JSON}`)
      );
    }

    return patterns;
  }

  async #createPackages(files: Path[]): Promise<MihoPackage[]> {
    const fullpaths = new Set<string>();
    const packages = await Promise.all(
      files.map(async (pathObj) => {
        const pkg = await MihoPackage.create(this, pathObj, this.#config);
        if (!pkg) return null;

        if (fullpaths.has(pkg.fullpath)) return null;
        fullpaths.add(pkg.fullpath);
        return pkg;
      })
    );

    return packages.filter(Boolean) as MihoPackage[];
  }

  async #setPackageManager(cwd: string = process.cwd()) {
    if (!isPackageManager(this.#config.packageManager)) {
      this.#config.packageManager = await detectPackageManager({ cwd });
    }
  }

  #createExecaOptions(cwd: string = process.cwd()) {
    let options: ExecaOptions = { cwd };
    if (this.#config.verbose) {
      options = { ...options, stdio: 'inherit' };
    }

    return options;
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
