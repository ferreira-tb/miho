import { getDefaultOptions } from './options';
import { type FileData, Miho } from '../../src';

export function toHaveBeenBumped(testName: string) {
  async function fn(oldPkgs: FileData[]) {
    const options = getDefaultOptions(testName);
    const updatedMiho = await new Miho(options).search();
    const updatedPkgs = updatedMiho.getPackages();

    function bumped(pkg: FileData) {
      const old = oldPkgs.find(({ name }) => name === pkg.name);
      if (!old) return false;
      return pkg.version === old.newVersion;
    }

    return {
      pass: updatedPkgs.every(bumped),
      message: () => {
        const bumpedAmount = updatedPkgs.filter(bumped).length;
        return `${bumpedAmount} package(s) bumped`;
      }
    };
  }

  return fn;
}
