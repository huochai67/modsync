use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, Mutex};

use crate::{
    error::Error,
    mstask::{DownloadTask, FileTask, TaskEvent, TaskEventType, UnZipTask},
};

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Download = 0,
    Rename = 1,
    Delete = 2,
    UnZip = 3,
    // 未来可扩展其他任务类型
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequest {
    pub name: String,
    pub file_path: String,
    pub task_type: TaskType,

    // 下载任务的 URL
    pub url: Option<String>,

    // 重命名任务的新路径
    pub new_path: Option<String>,

    /// Expected uppercase MD5 for downloaded content. Optional for backwards compatibility.
    pub expected_md5: Option<String>,
}
impl TaskRequest {
    pub fn download(name: String, url: String, file_path: String) -> Self {
        Self::download_verified(name, url, file_path, None)
    }

    pub fn download_verified(
        name: String,
        url: String,
        file_path: String,
        expected_md5: Option<String>,
    ) -> Self {
        Self {
            name,
            file_path,
            task_type: TaskType::Download,
            url: Some(url),
            new_path: None,
            expected_md5,
        }
    }

    pub fn delete(name: String, file_path: String) -> Self {
        Self {
            name,
            file_path,
            task_type: TaskType::Delete,
            url: None,
            new_path: None,
            expected_md5: None,
        }
    }

    pub fn rename(name: String, file_path: String, new_path: String) -> Self {
        Self {
            name,
            file_path,
            task_type: TaskType::Rename,
            url: None,
            new_path: Some(new_path),
            expected_md5: None,
        }
    }

    pub fn unzip(name: String, file_path: String, dir_path: String) -> Self {
        Self {
            name,
            file_path,
            task_type: TaskType::UnZip,
            url: None,
            new_path: Some(dir_path),
            expected_md5: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRunSummary {
    pub tasks: Vec<TaskStatus>,
    pub succeeded: usize,
    pub failed: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub id: usize,
    pub name: String,
    pub downloaded_bytes: Option<usize>,
    pub total_bytes: Option<usize>,
    pub error: Option<String>,
    pub status: TaskEventType,
}
impl TaskStatus {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            downloaded_bytes: None,
            total_bytes: None,
            error: None,
            status: TaskEventType::Started,
        }
    }
}

/// 管理类
pub struct TaskManager {
    max_concurrent: usize,
    client: reqwest::Client,

    vec_task_status: Arc<Mutex<Vec<TaskStatus>>>,
}
impl TaskManager {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            max_concurrent,
            client: reqwest::Client::new(),
            vec_task_status: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_max_concurrent(&self) -> usize {
        self.max_concurrent
    }

    pub async fn get_vec_task_status(&self) -> Arc<Mutex<Vec<TaskStatus>>> {
        self.vec_task_status.clone()
    }

    pub async fn run(&mut self, targets: Vec<TaskRequest>) -> Result<TaskRunSummary, Error> {
        let (tx, mut rx) = mpsc::channel(100);
        let (job_tx, job_rx) = mpsc::unbounded_channel();
        let job_rx = Arc::new(Mutex::new(job_rx));

        for (i, task) in targets.into_iter().enumerate() {
            self.vec_task_status
                .lock()
                .await
                .push(TaskStatus::new(i, task.name.clone()));
            job_tx.send((i, task)).map_err(|_| Error::MSTaskMPSC)?;
        }
        drop(job_tx);

        let mut workers = Vec::with_capacity(self.max_concurrent);
        for _ in 0..self.max_concurrent {
            let job_rx = job_rx.clone();
            let tx_clone = tx.clone();
            let client = self.client.clone();
            // Fixed-size worker pool: task count no longer determines spawned task count.
            workers.push(tokio::spawn(async move {
                loop {
                    let Some((i, task)) = job_rx.lock().await.recv().await else {
                        break;
                    };
                    match task.task_type {
                        TaskType::Download => {
                            let Some(url) = task.url else {
                                let _ = tx_clone
                                    .send(TaskEvent::error(
                                        i,
                                        "download task has no URL".to_string(),
                                    ))
                                    .await;
                                continue;
                            };
                            let task = DownloadTask::new(
                                i,
                                url,
                                task.file_path,
                                task.expected_md5,
                                client.clone(),
                                tx_clone.clone(),
                            );
                            if let Err(e) = task.execute().await {
                                let _ = tx_clone.send(TaskEvent::error(i, e.to_string())).await;
                            }
                        }
                        TaskType::Delete => {
                            let task = FileTask::delete(i, task.file_path, tx_clone.clone());
                            if let Err(e) = task.execute().await {
                                let _ = tx_clone.send(TaskEvent::error(i, e.to_string())).await;
                            }
                        }
                        TaskType::Rename => {
                            let Some(new_path) = task.new_path else {
                                let _ = tx_clone
                                    .send(TaskEvent::error(
                                        i,
                                        "rename task has no destination".to_string(),
                                    ))
                                    .await;
                                continue;
                            };
                            let task =
                                FileTask::rename(i, task.file_path, new_path, tx_clone.clone());
                            if let Err(e) = task.execute().await {
                                let _ = tx_clone.send(TaskEvent::error(i, e.to_string())).await;
                            }
                        }
                        TaskType::UnZip => {
                            let Some(destination) = task.new_path else {
                                let _ = tx_clone
                                    .send(TaskEvent::error(
                                        i,
                                        "unzip task has no destination".to_string(),
                                    ))
                                    .await;
                                continue;
                            };
                            let task =
                                UnZipTask::new(i, task.file_path, destination, tx_clone.clone());
                            if let Err(e) = task.execute().await {
                                let _ = tx_clone.send(TaskEvent::error(i, e.to_string())).await;
                            }
                        }
                    }
                }
            }));
        }

        // 显式丢弃掉最初的 tx，否则 rx.recv() 永远不会结束
        drop(tx);

        // 统一处理来自所有任务的消息
        while let Some(event) = rx.recv().await {
            if let Some(task_info) = self
                .vec_task_status
                .lock()
                .await
                .iter_mut()
                .find(|t| t.id == event.id)
            {
                task_info.status = event.event_type;
                if let Some(download) = event.downloaded {
                    task_info.downloaded_bytes = Some(download)
                }
                if let Some(total) = event.total {
                    task_info.total_bytes = Some(total)
                }
                if let Some(error) = event.error_message {
                    task_info.error = Some(error)
                }
            };
        }

        for worker in workers {
            worker.await?;
        }

        let tasks = self.vec_task_status.lock().await.clone();
        let failed = tasks
            .iter()
            .filter(|task| matches!(task.status, TaskEventType::Error))
            .count();
        let succeeded = tasks
            .iter()
            .filter(|task| matches!(task.status, TaskEventType::Finished))
            .count();
        Ok(TaskRunSummary {
            tasks,
            succeeded,
            failed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn failed_file_task_is_reported_in_summary() {
        let mut manager = TaskManager::new(1);
        let summary = manager
            .run(vec![TaskRequest::delete(
                "missing".to_string(),
                "this-file-does-not-exist".to_string(),
            )])
            .await
            .expect("task manager should report task failures, not crash");

        assert_eq!(summary.succeeded, 0);
        assert_eq!(summary.failed, 1);
        assert!(matches!(summary.tasks[0].status, TaskEventType::Error));
        assert!(summary.tasks[0].error.is_some());
    }
}
