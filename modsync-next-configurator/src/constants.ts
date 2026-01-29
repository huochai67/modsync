
import { ReleaseInfo } from './types';

export const MOCK_HISTORY: ReleaseInfo[] = [
    {
        version: "v1.2.5",
        changelog: "Optimization of core engine and bug fixes for the UI renderer.",
        date: "2024-05-20",
        adds: ["engine/core_v2.bin", "assets/splash_new.png"],
        subs: ["engine/core_v1.bin"],
        mods: ["ui/theme.json", "config/settings.yaml"],
        size: 10485760 // 10MB
    },
    {
        version: "v1.2.4",
        changelog: "Added support for external plugins and multi-language localization.",
        date: "2024-04-12",
        adds: ["plugins/sdk.lib", "locales/zh-CN.json", "locales/en-US.json"],
        mods: ["main.exe"],
        size: 5242880 // 5MB
    },
    {
        version: "v1.2.0",
        changelog: "Major feature update: Real-time data synchronization.",
        date: "2024-03-01",
        adds: ["sync_module.dll", "docs/api_guide.pdf"],
        subs: ["legacy_sync.dll"],
        mods: ["app.config"],
        size: 25165824 // 24MB
    }
];

export const formatBytes = (bytes?: number) => {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};
