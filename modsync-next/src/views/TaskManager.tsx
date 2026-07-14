import React, { useEffect } from "react";
import TaskProgress from "@/components/TaskProgress";
import { invoke } from "@tauri-apps/api/core";

import { TaskStatus } from "@/types";
import { ListChecks } from "lucide-react";

const TaskManager: React.FC = () => {
  const [task_status, setTaskStatus] = React.useState<Array<TaskStatus>>([]);
  useEffect(() => {
    const interval = window.setInterval(async () => {
      try {
        // const is_running = await invoke<boolean>('is_running');
        // if (!is_running) {
        //     throw new Error("No task is running");
        // }
        let running_tasks = await invoke<TaskStatus[]>("getall_task");
        setTaskStatus(running_tasks);
      } catch (error) {
        // alert("Failed to fetch TaskInfo: " + error);
        // window.location.href = "http://localhost:1420/"
      }
    }, 1000);
    return () => window.clearInterval(interval);
  }, []);

  return (
    <div className="page-wrap w-full h-full">
      <header className="mb-6"><div className="eyebrow">Sync activity</div><h1 className="page-heading">任务管理</h1><p className="page-description">实时查看下载、删除和文件处理进度。</p></header>
      {task_status.length === 0 ? (
        <div className="task-empty surface">
          <div><div className="empty-icon"><ListChecks size={24}/></div><p className="font-semibold">当前没有任务记录</p><p className="page-description mt-2">开始同步后，任务进度会显示在这里。</p></div>
        </div>
      ) : (
        <div className="flex flex-col gap-4 min-h-full">
          {task_status.map((task) => (
            <TaskProgress key={task.id} taskStatus={task} />
          ))}
        </div>
      )}
    </div>
  );
};

export default TaskManager;
