import React, { useEffect } from "react";
import TaskProgress from "@/components/TaskProgress";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

import { TaskStatus } from "@/types";
import { ListChecks } from "lucide-react";
import { SYNC_STATE_EVENT, TASK_PROGRESS_EVENT } from "@modsync/contracts";

const TaskManager: React.FC = () => {
  const [task_status, setTaskStatus] = React.useState<Array<TaskStatus>>([]);
  useEffect(() => {
    let disposed = false;
    const unlisteners: UnlistenFn[] = [];

    const subscribe = async () => {
      const stopSyncState = await listen<boolean>(SYNC_STATE_EVENT, (event) => {
        if (!disposed && event.payload) setTaskStatus([]);
      });
      if (disposed) {
        stopSyncState();
        return;
      }
      unlisteners.push(stopSyncState);

      const stopTaskProgress = await listen<TaskStatus>(
        TASK_PROGRESS_EVENT,
        (event) => {
          if (disposed) return;
          setTaskStatus((current) => {
            const index = current.findIndex((task) => task.id === event.payload.id);
            if (index === -1) {
              return [...current, event.payload].sort((a, b) => a.id - b.id);
            }
            const next = [...current];
            next[index] = event.payload;
            return next;
          });
        },
      );
      if (disposed) {
        stopTaskProgress();
        return;
      }
      unlisteners.push(stopTaskProgress);
      try {
        const runningTasks = await invoke<TaskStatus[]>("getall_task");
        if (!disposed) setTaskStatus(runningTasks);
      } catch (error) {
        console.error("Failed to load task state", error);
      }
    };

    subscribe().catch((error) => {
      console.error("Failed to subscribe to task progress", error);
    });
    return () => {
      disposed = true;
      unlisteners.forEach((unlisten) => unlisten());
    };
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
