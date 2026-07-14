export type ChangeType = "added" | "modified" | "deleted" | "renamed";

import type { ReleaseInfo } from "@modsync/contracts";

export {
  DiffType,
  Kind,
  TaskEventType,
  TaskType,
  type MODDiff,
  type MSConfig,
  type MSMOD,
  type MetaData,
  type ReleaseInfo,
  type TaskRequest,
  type TaskRunSummary,
  type TaskStatus,
} from "@modsync/contracts";

export interface FileDiff {
  id: string;
  path: string;
  status: ChangeType;
  sizeBefore: number;
  sizeAfter: number;
  md5Before: string;
  md5After: string;
  linesAdded: number;
  linesDeleted: number;
}

export interface RuntimeInfo {
  title: string;
  version: string;
  buildinfo: string;
  has_serverdat: boolean;
  has_options: boolean;
  has_hcml: boolean;
  has_pclce: boolean;
  has_configpack: boolean;
  release_info: ReleaseInfo[];
}

