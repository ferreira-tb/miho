import { MihoPackage } from './package';
import { FileType } from '../utils/enum';
import type { MihoInternalOptions } from '../miho';

export class FileData {
  public readonly fullpath: string;
  public readonly id: number;
  public readonly name: string | null;
  public readonly newVersion: string | null;
  public readonly release: MihoInternalOptions['release'];
  public readonly type: FileType;
  public readonly version: string;

  constructor(id: number, pkg: MihoPackage) {
    this.id = id;
    this.type = FileType.PACKAGE_JSON;
    this.name = pkg.packageName;
    this.version = pkg.version;
    this.newVersion = pkg.newVersion;
    this.fullpath = pkg.fullpath;
    this.release = pkg.release;
  }
}
