/**
 * Public data contract shared by ModSync desktop applications.
 *
 * The Rust implementation accepts configuration documents without
 * `schema_version` as version 1 for backwards compatibility.
 */
export const CONFIG_SCHEMA_VERSION = 1 as const;
export const SYNC_STATE_EVENT = "modsync://sync-state" as const;
export const TASK_PROGRESS_EVENT = "modsync://task-progress" as const;

export interface MSMOD {
  md5: string;
  path: string;
  size: number;
  url?: string;
  modid?: string;
  version?: string;
}

export interface MetaData {
  options_url?: string;
  serverdat_url?: string;
  configpack?: MSMOD;
  launcher_hmcl_url?: string;
  launcher_pclce_url?: string;
}

export interface ReleaseInfo {
  version: string;
  changelog: string;
  date: string;
  adds?: string[];
  subs?: string[];
  mods?: string[];
  size?: number;
}

export interface MSConfig {
  schema_version: number;
  base_url: string;
  release_info: ReleaseInfo[];
  modlist_url?: string;
  metadata?: MetaData;
  title: string;
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

export enum TaskType {
  Download = "Download",
  Rename = "Rename",
  Delete = "Delete",
  UnZip = "UnZip",
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
  url?: string;
  new_path?: string;
  expected_md5?: string;
}
