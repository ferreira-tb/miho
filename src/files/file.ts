import { MihoPackage } from './package';
import { FileType } from '../utils/enum';

export class FileData {
  public readonly id: number;
  public readonly name: string | null;
  public readonly newVersion: string | null;
  public readonly type: FileType;
  public readonly version: string;

  constructor(id: number, pkg: MihoPackage) {
    this.id = id;
    this.type = FileType.PACKAGE_JSON;
    this.name = pkg.packageName;
    this.version = pkg.version;
    this.newVersion = pkg.newVersion;
  }
}
