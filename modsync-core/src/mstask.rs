use futures_util::StreamExt;
use std::path::Path;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;

use crate::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskEventType {
    Started,
    Progress,
    Finished,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvent {
    pub event_type: TaskEventType,
    pub id: usize,
    pub downloaded: Option<u64>,
    pub total: Option<u64>,
    pub error_message: Option<String>,
}

impl TaskEvent {
    pub fn started(id: usize) -> Self {
        Self {
            event_type: TaskEventType::Started,
            id,
            downloaded: None,
            total: None,
            error_message: None,
        }
    }

    pub fn progress(id: usize, downloaded: u64, total: u64) -> Self {
        Self {
            event_type: TaskEventType::Progress,
            id,
            downloaded: Some(downloaded),
            total: Some(total),
            error_message: None,
        }
    }

    pub fn finished(id: usize) -> Self {
        Self {
            event_type: TaskEventType::Finished,
            id,
            downloaded: None,
            total: None,
            error_message: None,
        }
    }

    pub fn error(id: usize, message: String) -> Self {
        Self {
            event_type: TaskEventType::Error,
            id,
            downloaded: None,
            total: None,
            error_message: Some(message),
        }
    }
}

pub struct FileTask {
    id: usize,
    file_path: String,
    new_path: Option<String>,
    tx: mpsc::Sender<TaskEvent>,
}

impl FileTask {
    pub fn new(
        id: usize,
        file_path: String,
        new_path: Option<String>,
        tx: mpsc::Sender<TaskEvent>,
    ) -> Self {
        Self {
            id,
            file_path,
            new_path,
            tx,
        }
    }

    pub fn delete(id: usize, file_path: String, tx: mpsc::Sender<TaskEvent>) -> Self {
        Self {
            id,
            file_path,
            new_path: None,
            tx,
        }
    }

    pub fn rename(
        id: usize,
        file_path: String,
        new_path: String,
        tx: mpsc::Sender<TaskEvent>,
    ) -> Self {
        Self {
            id,
            file_path,
            new_path: Some(new_path),
            tx,
        }
    }

    pub async fn execute(self) -> Result<(), Error> {
        self.tx.send(TaskEvent::started(self.id)).await?;

        let file_path = Path::new(&self.file_path);
        if !file_path.exists() {
            return Err(Error::from(tokio::io::Error::new(
                tokio::io::ErrorKind::AddrNotAvailable,
                "File path does not exist",
            )));
        }

        match self.new_path {
            Some(new_path) => fs::rename(file_path, new_path).await?,
            None => {
                fs::remove_file(file_path).await?;
            }
        }

        self.tx.send(TaskEvent::finished(self.id)).await?;
        Ok(())
    }
}

pub struct DownloadTask {
    id: usize,
    url: String,
    file_path: String,
    client: reqwest::Client,
    tx: mpsc::Sender<TaskEvent>,
}
impl DownloadTask {
    pub fn new(
        id: usize,
        url: String,
        file_path: String,
        client: reqwest::Client,
        tx: mpsc::Sender<TaskEvent>,
    ) -> Self {
        Self {
            id,
            url,
            file_path,
            client,
            tx,
        }
    }
    pub async fn execute(self) -> Result<(), Error> {
        // 1. 发起请求
        let response = self.client.get(&self.url).send().await?;
        let total_size = response.content_length().unwrap_or(0);
        self.tx.send(TaskEvent::started(self.id)).await?;
        // 2. 创建文件
        let mut file = File::create(&self.file_path).await?;
        let mut downloaded: u64 = 0;
        // 3. 流式读取，避免大文件占用内存
        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).await?;

            downloaded += chunk.len() as u64;
            // 4. 发送进度 (对于100KB的小文件，这会非常快)
            let _ = self
                .tx
                .send(TaskEvent::progress(self.id, downloaded, total_size))
                .await;
        }
        file.flush().await?;
        self.tx.send(TaskEvent::finished(self.id)).await?;
        Ok(())
    }
}
