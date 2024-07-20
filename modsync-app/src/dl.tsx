import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { Listbox, ListboxItem, Progress } from "@nextui-org/react";
import { useTranslation } from 'react-i18next';
import { mb_error, mb_info } from "./utils/messagebox";
import "./utils/i18n"

import "./global.css";

type TaskInfo = {
    totalsize: number,
    downloadsize: number,
    name: string,
}
type GetTaskPayload = {
    tasks: Array<TaskInfo>,
    num_total: number,
    num_finished: number,
}

function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

export function Page() {
    const { t } = useTranslation();
    const [tasklist, settasklist] = useState<GetTaskPayload>({ tasks: [], num_total: 0, num_finished: 0 });

    function fetchtasks() {
        invoke<GetTaskPayload>('get_tasks').then((value) => {
            console.log(value);
            settasklist(value);
            sleep(50).then(() => {
                if (value.tasks.length == 0) {
                    mb_info(t("DONE"));
                    window.location.replace('/')
                } else {
                    fetchtasks()
                }
            });
        }).catch(mb_error);
    }

    useEffect(fetchtasks, []);

    return (
        <div className="flex flex-col h-full border-4 divide-y-4 divide-background border-background text-foreground bg-background">
            <Listbox items={tasklist.tasks}>
                {(item) => (
                    <ListboxItem key={item.name}>
                        <Progress label={item.name} showValueLabel value={(item.downloadsize / item.totalsize) * 100} />
                    </ListboxItem>
                )}
            </Listbox>
        </div>
    );
}

export default Page;
