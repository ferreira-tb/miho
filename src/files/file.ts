import { MihoPackage } from './package';
import { FileType } from '../utils/enum';

export class FileData {
  readonly id: number;
  readonly type: FileType;
  readonly name: string | null;
  readonly version: string;
  readonly newVersion: string | null;

  constructor(id: number, pkg: MihoPackage) {
    this.id = id;
    this.type = FileType.PACKAGE_JSON;
    this.name = pkg.packageName;
    this.version = pkg.version;
    this.newVersion = pkg.newVersion;
  }
}
