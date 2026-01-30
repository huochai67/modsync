
import React from 'react';

import { RuntimeContext } from '../runtimecontext';
import { Button, Card, Disclosure, Separator } from '@heroui/react';
import { invoke } from '@tauri-apps/api/core';
import { MOCK_TASK_REQUESTS } from '@/mockData';
import { MODDiff, TaskStatus } from '@/types';

const TestPage: React.FC = () => {
    const runtime = React.useContext(RuntimeContext);

    const [isExpanded, setIsExpanded] = React.useState(false);

    return (
        <div className='w-full min-h-full flex flex-col gap-6 overflow-y-auto'>
            <Disclosure className='w-full' isExpanded={isExpanded} onExpandedChange={setIsExpanded}>
                <Disclosure.Heading>
                    <Button slot="trigger" variant="secondary">
                        Runtime
                        <Disclosure.Indicator />
                    </Button>
                </Disclosure.Heading>
                <Disclosure.Content>
                    {JSON.stringify(runtime)}
                </Disclosure.Content>
            </Disclosure>
            <Card className='w-full'>
                <Card.Header>
                    <Card.Title>任务功能测试</Card.Title>
                </Card.Header>
                <Separator />
                <Card.Content className='flex flex-row flex-wrap gap-3'>
                    <Button onClick={async () => {
                        console.log('提交任务：', MOCK_TASK_REQUESTS);
                        await invoke('summit_task', { tasks: MOCK_TASK_REQUESTS });
                        alert('任务完成！');
                    }}>提交任务</Button>
                    <Button onClick={async () => {
                        console.log('获取任务列表');
                        let running_tasks = await invoke<TaskStatus[]>('getall_task')
                        console.log(running_tasks);
                    }}>获取列表</Button>
                    <Button onClick={async () => {
                        console.log('获取状态');
                        let is_running = await invoke<boolean>('is_running')
                        console.log('状态：', is_running);
                    }}>ISRUNNING</Button>
                    <Button onClick={async () => {
                        const diff = await invoke<MODDiff[]>('get_diff');
                        console.log('获取文件差异：', diff);
                    }}>MODDIFF</Button>
                </Card.Content>
            </Card>
        </div >

    );
};

export default TestPage;
