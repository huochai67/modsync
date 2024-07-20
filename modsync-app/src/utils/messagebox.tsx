import { message } from '@tauri-apps/plugin-dialog';
import { exit } from '@tauri-apps/plugin-process';

export async function mb_error(msg: string, title: string = "MODSYNC") {
    await message(msg, { title: title, kind: 'error' });
    return await exit(0);
}

export function mb_info(msg: string, title: string = "MODSYNC") {
    return message(msg, { title: title, kind: 'info' });
}