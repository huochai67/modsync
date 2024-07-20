import { useEffect, useState } from "react";
import { invoke } from '@tauri-apps/api/core';
import { Card, CardBody, CardFooter, CardHeader, Divider, Listbox, ListboxItem, Progress, Spinner } from "@nextui-org/react";
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
    const [gtpayload, settasklist] = useState<GetTaskPayload>({ tasks: [], num_total: 0, num_finished: 0 });

    function fetchtasks() {
        invoke<GetTaskPayload>('get_tasks').then((value) => {
            settasklist(value);
            sleep(50).then(() => {
                if (value.tasks.length == 0) {
                    mb_info(t("DONE")).then(() => window.location.replace('/'));
                } else {
                    fetchtasks()
                }
            });
        }).catch(mb_error);
    }

    useEffect(fetchtasks, []);

    return (
        <div className="flex flex-col h-full border-4 divide-y-4 divide-background border-background text-foreground bg-background">
            <Card className="h-full">
                <CardHeader className="divide-x-8 divide-foreground-50">
                    <Spinner size="sm" aria-label="sp-dl" />
                    <p>{t('DOWNLOADING')}</p>
                </CardHeader>
                <Divider />
                <CardBody className="grow">
                    <Listbox items={gtpayload.tasks}>
                        {(item) => (
                            <ListboxItem key={item.name}>
                                <Progress maxValue={item.totalsize} label={item.name} showValueLabel value={item.downloadsize} />
                            </ListboxItem>
                        )}
                    </Listbox>
                </CardBody>
                <Divider />
                <CardFooter>
                    <div className="flex flex-row w-full h-6 text-nowrap content-center items-center divide-x-8 divide-foreground-50">
                        <p>{t('PB_TOTAL')}</p>
                        <Progress value={(gtpayload.num_finished / gtpayload.num_total) * 100} />
                        <p>{gtpayload.num_finished} / {gtpayload.num_total}</p>
                    </div>
                </CardFooter>
            </Card>
        </div>
    );
}

export default Page;
