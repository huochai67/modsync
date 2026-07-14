export type ChangeType = "added" | "modified" | "deleted" | "renamed";

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

export interface MSMOD {
  md5: string;
  path: string;
  size: number;
  url?: string;
  modid?: string;
  version?: string;
}

export enum Kind {
  PLAIN = "PLAIN",
  MOD = "MOD",
}
export enum DiffType {
  NEWED = "NEWED",
  DELETED = "DELETED",
  MODIFIED = "MODIFIED",
}
export interface MODDiff {
  kind: Kind;
  name: string;
  difftype: DiffType;
  local?: MSMOD;
  remote?: MSMOD;
}
export interface ReleaseInfo {
  version: string;
  changelog: string;
  date: string;
  adds?: string[];
  subs?: string[];
  mods?: string[];
  size?: number; // bytes
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

export enum TaskType {
  Download = "Download",
  Rename = "Rename",
  Delete = "Delete",
}

export enum TaskEventType {
  Started = "Started",
  Progress = "Progress",
  Finished = "Finished",
  Error = "Error",
}

export interface TaskStatus {
  id: number;
  name: string;
  downloaded_bytes?: number;
  total_bytes?: number;
  error?: string;
  status: TaskEventType;
}

export interface TaskRunSummary {
  tasks: TaskStatus[];
  succeeded: number;
  failed: number;
}

export interface TaskRequest {
  name: string;
  file_path: string;
  task_type: TaskType;

  // 下载任务的URL
  url?: string;

  // 重命名任务的新路径
  new_path?: string;
}
