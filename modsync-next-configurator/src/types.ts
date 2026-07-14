
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
    metadata?: MetaData;
    title: string;
}

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

export interface FormState {
    version: string;
    changelog: string;
    serverUrl: string;
    title: string;
    adds: string[];
    subs: string[];
    mods: string[];
}
