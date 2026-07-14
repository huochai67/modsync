import {
  FileDiff,
  RuntimeInfo,
  TaskRequest,
  ReleaseInfo,
  TaskType,
  MODDiff,
  Kind,
  MSMOD,
  DiffType,
} from "./types";

export const MOCK_DIFFS: FileDiff[] = [
  {
    id: "1",
    path: "src/core/diff-engine.ts",
    status: "modified",
    sizeBefore: 12450,
    sizeAfter: 13200,
    md5Before: "e99a18c428cb38d5f260853678922e03",
    md5After: "d41d8cd98f00b204e9800998ecf8427e",
    linesAdded: 120,
    linesDeleted: 45,
  },
  {
    id: "2",
    path: "src/ui/components/Header.tsx",
    status: "added",
    sizeBefore: 0,
    sizeAfter: 4500,
    md5Before: "00000000000000000000000000000000",
    md5After: "c4ca4238a0b923820dcc509a6f75849b",
    linesAdded: 85,
    linesDeleted: 0,
  },
  {
    id: "3",
    path: "assets/legacy-icon.png",
    status: "deleted",
    sizeBefore: 45200,
    sizeAfter: 0,
    md5Before: "25d55ad283aa400af464c76d713c07ad",
    md5After: "00000000000000000000000000000000",
    linesAdded: 0,
    linesDeleted: 0,
  },
  {
    id: "4",
    path: "config/settings.json",
    status: "modified",
    sizeBefore: 1200,
    sizeAfter: 1250,
    md5Before: "827ccb0eea8a706c4c34a16891f84e7b",
    md5After: "0cc175b9c0f1b6a831c399e269772661",
    linesAdded: 5,
    linesDeleted: 2,
  },
  {
    id: "5",
    path: "src/utils/md5-helper.ts",
    status: "renamed",
    sizeBefore: 2100,
    sizeAfter: 2100,
    md5Before: "60b725f10c9c85c70d97880dfe8191b3",
    md5After: "60b725f10c9c85c70d97880dfe8191b3",
    linesAdded: 0,
    linesDeleted: 0,
  },
];

export const MOCK_RELEASES: ReleaseInfo[] = [
  {
    version: "2.1.0",
    changelog: "Minor improvements and bug fixes",
    date: "2024-01-15",
    adds: ["Fast sync mode", "Batch operations"],
    subs: ["Slow UI rendering"],
    mods: ["Improved diff algorithm", "Better error handling"],
    size: 52428800,
  },
  {
    version: "2.0.5",
    changelog: "Critical security update",
    date: "2024-01-10",
    adds: ["Security patches"],
    mods: ["Fixed vulnerability in file handling"],
    size: 51380224,
  },
  {
    version: "2.0.0",
    changelog: "Major release with redesigned UI",
    date: "2023-12-20",
    adds: ["New dashboard", "Real-time sync monitoring", "Advanced filtering"],
    subs: ["Legacy configuration format"],
    mods: ["Complete backend rewrite"],
    size: 54525952,
  },
];

export const MOCK_TASK_REQUESTS: TaskRequest[] = [
  {
    name: "QQ NT Installer",
    file_path: "../qq.exe",
    task_type: TaskType.Download,
    url: "https://dldir1v6.qq.com/qqfile/qq/QQNT/Windows/QQ_9.9.26_260116_x64_01.exe",
  },
];

export const MOCK_MODS: MSMOD[] = [
  {
    md5: "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6",
    path: "mods/ImmersiveEngineering-1.20.1-10.1.0.jar",
    size: 15728640,
    url: "https://www.curseforge.com/minecraft/mods/immersive-engineering",
    modid: "immersiveengineering",
    version: "10.1.0",
  },
  {
    md5: "b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7",
    path: "mods/ThermalExpansion-1.20.1-10.2.5.jar",
    size: 8388608,
    url: "https://www.curseforge.com/minecraft/mods/thermal-expansion",
    modid: "thermalexpansion",
    version: "10.2.5",
  },
  {
    md5: "c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8",
    path: "mods/JourneyMap-1.20.1-5.9.11.jar",
    size: 6291456,
    url: "https://www.curseforge.com/minecraft/mods/journeymap",
    modid: "journeymap",
    version: "5.9.11",
  },
];

export const MOCK_MOD_DIFFS: MODDiff[] = [
  {
    kind: Kind.MOD,
    name: "ImmersiveEngineering",
    difftype: DiffType.MODIFIED,
    local: MOCK_MODS[0],
    remote: {
      md5: "x1y2z3a4b5c6d7e8f9g0h1i2j3k4l5m6",
      path: "mods/ImmersiveEngineering-1.20.1-10.2.0.jar",
      size: 16252928,
      url: "https://www.curseforge.com/minecraft/mods/immersive-engineering",
      modid: "immersiveengineering",
      version: "10.2.0",
    },
  },
  {
    kind: Kind.MOD,
    name: "ThermalExpansion",
    difftype: DiffType.NEWED,
    remote: MOCK_MODS[1],
  },
  {
    kind: Kind.MOD,
    name: "NewModToAdd",
    difftype: DiffType.NEWED,
    remote: {
      md5: "n1e2w3m4o5d6l7o8c9a0l1l2y3a4d5d6",
      path: "mods/NewMod-1.20.1-2.1.0.jar",
      size: 4194304,
      url: "https://www.curseforge.com/minecraft/mods/newmod",
      modid: "newmod",
      version: "2.1.0",
    },
  },
  {
    kind: Kind.PLAIN,
    name: "config.yml",
    difftype: DiffType.DELETED,
    local: {
      md5: "c1o2n3f4i5g6l7o8c9a1l2.",
      path: "config/config.yml",
      size: 2048,
    },
  },
];

export const MOCK_RUNTIME_INFO: RuntimeInfo = {
  title: "ModSync Next",
  version: "2.1.0",
  buildinfo: "build-20260130-001",
  release_info: MOCK_RELEASES,
  has_serverdat: false,
  has_options: false,
  has_hcml: false,
  has_pclce: false,
  has_configpack: false,
};
