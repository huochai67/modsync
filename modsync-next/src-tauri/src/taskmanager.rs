use std::sync::Arc;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, Mutex, Semaphore};

use crate::task::{DownloadTask, FileTask, TaskEvent, TaskEventType};

#[repr(u8)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskType {
    Download = 0,
    Rename = 1,
    Delete = 2,
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
}
impl TaskRequest {
    pub fn download(name: String, url: String, file_path: String) -> Self {
        Self {
            name,
            file_path,
            task_type: TaskType::Download,
            url: Some(url),
            new_path: None,
        }
    }

    pub fn delete(name: String, file_path: String) -> Self {
        Self {
            name,
            file_path,
            task_type: TaskType::Delete,
            url: None,
            new_path: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStatus {
    pub id: usize,
    pub name: String,
    pub downloaded_bytes: Option<u64>,
    pub total_bytes: Option<u64>,
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
    semaphore: Arc<Semaphore>,
    client: reqwest::Client,

    vec_task_status: Arc<Mutex<Vec<TaskStatus>>>,
}
impl TaskManager {
    pub fn new(max_concurrent: usize) -> Self {
        Self {
            max_concurrent,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            client: reqwest::Client::new(),
            vec_task_status: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn get_vec_task_status(&self) -> Arc<Mutex<Vec<TaskStatus>>> {
        self.vec_task_status.clone()
    }

    pub async fn run(&mut self, targets: Vec<TaskRequest>) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(100);

        // 产生任务

        for (i, task) in targets.into_iter().enumerate() {
            let tx_clone = tx.clone();
            let semaphore = self.semaphore.clone();
            let client = self.client.clone();
            self.vec_task_status
                .lock()
                .await
                .push(TaskStatus::new(i, task.name.clone()));
            let task = Arc::new(task);

            // 每个任务创建一个独立的 Task 实例并 spawn 到 Tokio 运行时
            tokio::spawn(async move {
                // 获取信号量许可，控制并发
                let _permit = semaphore.acquire().await.unwrap();

                match task.task_type {
                    TaskType::Download => {
                        let url = task.url.clone().unwrap();
                        let path = task.file_path.clone();
                        let task = DownloadTask::new(i, url, path, client, tx_clone.clone());
                        if let Err(e) = task.execute().await {
                            let _ = tx_clone.send(TaskEvent::error(i, e.to_string())).await;
                        }
                    }
                    TaskType::Delete => {
                        let task = FileTask::delete(i, task.file_path.clone(), tx_clone.clone());
                        if let Err(e) = task.execute().await {
                            let _ = tx_clone.send(TaskEvent::error(i, e.to_string())).await;
                        }
                    }
                    TaskType::Rename => {
                        let new_path = task.new_path.clone().unwrap();
                        let task =
                            FileTask::rename(i, task.file_path.clone(), new_path, tx_clone.clone());
                        if let Err(e) = task.execute().await {
                            let _ = tx_clone.send(TaskEvent::error(i, e.to_string())).await;
                        }
                    }
                    _ => {
                        // 其他任务类型的处理逻辑
                        unimplemented!()
                    }
                };
            });
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

        Ok(())
    }
}

// #[tokio::main]
// async fn main() -> Result<()> {
//     // 模拟 4000 个任务
//     let mut targets = Vec::new();
//     for i in 0..4000 {
//         // 使用占位图片作为示例资源
//         targets.push((
//             "https://via.placeholder.com/100".to_string(),
//             format!("file_{}.png", i),
//         ));
//     }

//     // 设置最大并发数为 20
//     let manager = TaskManager::new(20);

//     println!("Starting download of {} files...", targets.len());
//     let start_time = std::time::Instant::now();

//     manager.run(targets).await?;

//     println!("All done in {:?}", start_time.elapsed());
//     Ok(())
// }
