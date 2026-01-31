
export interface ReleaseInfo {
    version: string;
    changelog: string;
    date: string;
    adds?: string[];
    subs?: string[];
    mods?: string[];
    size?: number; // bytes
}

export interface MSConfig {
    base_url: string;
    release_info: ReleaseInfo[];
    modlist_url?: string;
    option_url?: string;
    serverlist_url?: string;
    configpack?: any; // Placeholder for MSMOD type
    title: string;
}

export interface FormState {
    version: string;
    changelog: string;
    serverUrl: string;
    title: string;
    adds: string[];
    subs: string[];
    mods: string[];
}
