
export {
    type MSConfig,
    type MSMOD,
    type MetaData,
    type ReleaseInfo,
} from "@modsync/contracts";

export interface FormState {
    version: string;
    changelog: string;
    serverUrl: string;
    title: string;
    adds: string[];
    subs: string[];
    mods: string[];
}
