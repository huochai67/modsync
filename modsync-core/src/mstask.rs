use futures_util::StreamExt;
use std::path::{Component, Path, PathBuf};
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;
use tokio::sync::mpsc;
use zip::ZipArchive;

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
    pub downloaded: Option<usize>,
    pub total: Option<usize>,
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

    pub fn progress(id: usize, downloaded: usize, total: usize) -> Self {
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
            Some(new_path) => {
                if let Some(parent) = Path::new(&new_path).parent() {
                    fs::create_dir_all(parent).await?;
                }
                fs::rename(file_path, new_path).await?
            }
            None => {
                fs::remove_file(file_path).await?;
            }
        }

        self.tx.send(TaskEvent::finished(self.id)).await?;
        Ok(())
    }
}

pub struct UnZipTask {
    id: usize,
    file_path: String,
    dir_path: String,
    tx: mpsc::Sender<TaskEvent>,
}

impl UnZipTask {
    pub fn new(
        id: usize,
        file_path: String,
        dir_path: String,
        tx: mpsc::Sender<TaskEvent>,
    ) -> Self {
        Self {
            id,
            file_path,
            dir_path,
            tx,
        }
    }

    pub async fn execute(self) -> Result<(), Error> {
        self.tx.send(TaskEvent::started(self.id)).await?;

        let zip_path = self.file_path.clone();
        let dest_dir = self.dir_path.clone();

        let ziptx = self.tx.clone();
        tokio::task::spawn_blocking(move || -> Result<(), Error> {
            let file = std::fs::File::open(&zip_path)?;
            let mut archive = ZipArchive::new(file)?;

            let root = std::fs::canonicalize(&dest_dir).or_else(|_| {
                std::fs::create_dir_all(&dest_dir)?;
                std::fs::canonicalize(&dest_dir)
            })?;
            for i in 0..archive.len() {
                ziptx.try_send(TaskEvent::progress(self.id, i + 1, archive.len()))?;
                let mut file = archive.by_index(i)?;
                let entry_path = Path::new(file.name());
                if entry_path.is_absolute()
                    || entry_path.components().any(|part| {
                        matches!(
                            part,
                            Component::ParentDir | Component::RootDir | Component::Prefix(_)
                        )
                    })
                {
                    return Err(Error::Validation(format!(
                        "unsafe ZIP entry: {}",
                        file.name()
                    )));
                }
                let outpath: PathBuf = root.join(entry_path);

                if file.is_dir() {
                    std::fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            std::fs::create_dir_all(p)?;
                        }
                    }
                    let mut outfile = std::fs::File::create(&outpath)?;
                    std::io::copy(&mut file, &mut outfile)?;
                }
            }
            Ok(())
        })
        .await??;

        self.tx.send(TaskEvent::finished(self.id)).await?;
        Ok(())
    }
}

pub struct DownloadTask {
    id: usize,
    url: String,
    file_path: String,
    expected_md5: Option<String>,
    client: reqwest::Client,
    tx: mpsc::Sender<TaskEvent>,
}
impl DownloadTask {
    pub fn new(
        id: usize,
        url: String,
        file_path: String,
        expected_md5: Option<String>,
        client: reqwest::Client,
        tx: mpsc::Sender<TaskEvent>,
    ) -> Self {
        Self {
            id,
            url,
            file_path,
            expected_md5,
            client,
            tx,
        }
    }
    pub async fn execute(self) -> Result<(), Error> {
        // 1. 发起请求
        let response = self
            .client
            .get(&self.url)
            .send()
            .await?
            .error_for_status()?;
        let total_size = response.content_length().unwrap_or(0);
        self.tx.send(TaskEvent::started(self.id)).await?;
        // 2. 创建文件
        let target = Path::new(&self.file_path);
        let parent = target.parent().ok_or_else(|| {
            Error::Validation("download target has no parent directory".to_string())
        })?;
        fs::create_dir_all(parent).await?;
        let temporary = target.with_extension(format!(
            "{}.part",
            target
                .extension()
                .and_then(|v| v.to_str())
                .unwrap_or("download")
        ));
        let mut file = File::create(&temporary).await?;
        let mut downloaded: u64 = 0;
        let mut last_reported: u64 = 0;
        let mut digest = md5::Context::new();
        // 3. 流式读取，避免大文件占用内存
        let mut stream = response.bytes_stream();
        let result: Result<(), Error> = async {
            while let Some(item) = stream.next().await {
                let chunk = item?;
                file.write_all(&chunk).await?;
                digest.consume(&chunk);
                downloaded += chunk.len() as u64;
                // Limit updates to 128 KiB (and always report the final byte) to keep UI responsive.
                if downloaded - last_reported >= 128 * 1024 || downloaded == total_size {
                    let _ = self
                        .tx
                        .send(TaskEvent::progress(
                            self.id,
                            downloaded as usize,
                            total_size as usize,
                        ))
                        .await;
                    last_reported = downloaded;
                }
            }
            file.flush().await?;
            Ok(())
        }
        .await;
        drop(file);
        if let Err(error) = result {
            let _ = fs::remove_file(&temporary).await;
            return Err(error);
        }
        if let Some(expected) = &self.expected_md5 {
            let actual = format!("{:X}", digest.compute());
            if !actual.eq_ignore_ascii_case(expected) {
                let _ = fs::remove_file(&temporary).await;
                return Err(Error::Validation(format!(
                    "MD5 mismatch for {}: expected {}, got {}",
                    self.file_path, expected, actual
                )));
            }
        }
        if let Err(error) = fs::rename(&temporary, target).await {
            let _ = fs::remove_file(&temporary).await;
            return Err(error.into());
        }
        self.tx.send(TaskEvent::finished(self.id)).await?;
        Ok(())
    }
}
