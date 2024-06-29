import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { Listbox, ListboxItem, Progress } from "@nextui-org/react";
import { useTranslation } from 'react-i18next';
import { mb_error, mb_info } from "../messagebox";
import "../global.css";
import "../i18n"

type TaskInfo = {
    totalsize: number,
    downloadsize: number,
    name: string,
}

function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

function App() {
    const { t } = useTranslation();
    const [tasklist, settasklist] = useState(new Array<TaskInfo>());

    function fetchtasks() {
        invoke<TaskInfo[]>('get_tasks').then((value) => {
            settasklist(value);
            sleep(50).then(() => {
                if (value.length == 0) {
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
            <Listbox items={tasklist}>
                {(item) => (
                    <ListboxItem key={item.name}>
                        <Progress label={item.name} showValueLabel value={(item.downloadsize / item.totalsize) * 100} />
                    </ListboxItem>
                )}
            </Listbox>
        </div>
    );
}

export default App;
