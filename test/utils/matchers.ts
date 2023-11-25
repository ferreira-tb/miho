import type { ExpectStatic } from 'vitest';
import { getDefaultOptions } from './options';
import { Miho, type FileData } from '../../src';

type MatchersObject = Parameters<ExpectStatic['extend']>[0];
type MatcherState = ThisParameterType<MatchersObject[keyof MatchersObject]>;

export function toHaveBeenBumped(testName: string, thisArg?: MatcherState) {
  const fn = async function (oldPkgs: FileData[]) {
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
  };

  return fn.bind(thisArg);
}
