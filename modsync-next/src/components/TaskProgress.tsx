
import { Card } from '@heroui/react';
import React, { useEffect } from 'react';

import { TaskStatus } from '@/types';

interface TaskProgressProps {
    taskStatus: TaskStatus;
}

const TaskProgress: React.FC<TaskProgressProps> = ({ taskStatus }) => {
    const [progress, setProgress] = React.useState(0);
    useEffect(() => {
        if (taskStatus.status === 'Progress') {
            if (taskStatus.total_bytes == null || taskStatus.downloaded_bytes == null) {
                throw new Error("Invalid task status data");
            }
            const prog = (taskStatus.downloaded_bytes / taskStatus.total_bytes) * 100;
            setProgress(prog);
        } else if (taskStatus.status === 'Finished') {
            setProgress(100);
        } else {
            setProgress(0);
        }
    }, [taskStatus]);

    return (
        <Card className='min-h-20'>
            <Card.Header>
                <div className="flex items-center justify-between">
                    <h3 className="text-lg font-semibold">{taskStatus.name}</h3>
                    <span className="text-sm text-slate-400">{progress >= 100 ? "完成" : progress === 0 ? "等待" : `${progress.toFixed(1)}%`}</span>
                    {/* <span className="text-sm text-slate-400">{progress >= 100 ? "完成" : `${progress.toFixed(1)}%`}</span> */}
                </div>
            </Card.Header>
            <Card.Content>
                <div className="w-full bg-slate-200 rounded-full h-4 overflow-hidden">
                    <div
                        className="bg-indigo-500 h-4 transition-all"
                        style={{ width: `${progress}%` }}
                    ></div>
                </div>
            </Card.Content>
            <Card.Footer />

        </Card>
    );
};

export default TaskProgress;