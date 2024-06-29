import { message } from '@tauri-apps/plugin-dialog';
import { exit } from '@tauri-apps/plugin-process';

export function mb_error(msg: string, title: string = "MODSYNC") {
    message(msg, { title: title, kind: 'error' }).then(() => exit(0));
}

export function mb_info(msg: string, title: string = "MODSYNC") {
    message(msg, { title: title, kind: 'info' });
}