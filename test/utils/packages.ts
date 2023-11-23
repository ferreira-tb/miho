import { Miho, type MihoOptions, type PackageData } from '../../src';

export async function compareOldPackages(
  oldPkgs: PackageData[],
  options: Partial<MihoOptions>
) {
  const updatedMiho = await Miho.init(options);
  const updatedPkgs = updatedMiho.getPackages();
  for (const pkg of updatedPkgs) {
    const old = oldPkgs.find(({ name }) => name === pkg.name);
    if (!old) throw new TypeError(`Could not find package ${pkg.name}`);
    if (pkg.version !== old.newVersion) {
      throw new TypeError(
        `Version mismatch: ${pkg.version} !== ${old.newVersion}`
      );
    }
  }
}
