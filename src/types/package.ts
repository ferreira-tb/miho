export interface PackageOptions {
  /**
   * Prerelease identifier, like the `beta` in `1.0.0-beta.1`.
   * @default 'alpha'
   */
  preid: string;
  /**
   * This option will be applied to every package found by Miho.
   *
   * If this is a number, Miho will try to coerce it to a valid version.
   *
   * You can override this for individual packages in the config file.
   * @default 'patch'
   */
  release: string | number;
}
