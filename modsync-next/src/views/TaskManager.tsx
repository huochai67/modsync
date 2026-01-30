
import React, { useCallback, useEffect } from 'react';

import { RuntimeContext } from '../runtimecontext';
import TaskProgress from '@/components/TaskProgress';
import { invoke } from '@tauri-apps/api/core';

import { TaskStatus } from '@/types';

const TaskManager: React.FC = () => {
    const runtime = React.useContext(RuntimeContext);

    const [task_status, setTaskStatus] = React.useState<Array<TaskStatus>>([]);
    useEffect(() => {
        const _intervalId = setInterval(async () => {
            try {
                // const is_running = await invoke<boolean>('is_running');
                // if (!is_running) {
                //     throw new Error("No task is running");
                // }
                let running_tasks = await invoke<TaskStatus[]>('getall_task')
                console.log(running_tasks);
                setTaskStatus(running_tasks);
            } catch (error) {

                // alert("Failed to fetch TaskInfo: " + error);
                // window.location.href = "http://localhost:1420/"
            }
        }, 1000); // 1000 milliseconds = 1 second
    }, []);


    return (
        <div className='w-full h-full'>
            {task_status.length === 0 ?
                (
                    <div className="text-center py-20 bg-background-tertiary rounded-2xl border-2 border-accent">
                        <p>当前没有任务在运行。</p>
                    </div>
                ) : (
                    <div className='flex flex-col gap-4 min-h-full'>
                        {task_status.map((task) => <TaskProgress key={task.id} taskStatus={task} />)}
                    </div>
                )}
        </div>
    );
};

export default TaskManager;
