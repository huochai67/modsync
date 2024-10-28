use crate::error::Error;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use futures::StreamExt;
use tokio::sync::mpsc::Sender;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Clone)]
pub struct MSTaskStatus {
    pub name: String,
    pub total: u64,
    pub now: u64,
    pub finish: bool,
}

impl MSTaskStatus {
    pub fn new(name: String, total: u64, now: u64, finish: bool) -> Self {
        MSTaskStatus {
            name,
            total,
            now,
            finish,
        }
    }
}

#[async_trait]
pub trait MSTask {
    async fn start(&mut self, receiver: Sender<MSTaskStatus>) -> Result<(), Error>;
}

pub struct DownloadTask {
    reqclient: reqwest::Client,
    name: String,
    url: String,
    savepath: String,
}

impl DownloadTask {
    pub fn build(
        reqclient: reqwest::Client,
        name: String,
        url: String,
        savepath: String,
    ) -> DownloadTask {
        DownloadTask {
            reqclient,
            name,
            url,
            savepath,
        }
    }
}

#[async_trait]
impl MSTask for DownloadTask {
    async fn start(&mut self, receiver: Sender<MSTaskStatus>) -> Result<(), Error> {
        let path = Path::new(self.savepath.as_str()).parent().unwrap();
        tokio::fs::create_dir_all(path).await?;

        let mut save_file = File::create(self.savepath.as_str()).await?;

        let resp = self.reqclient.get(self.url.as_str()).send().await?;
        let totalsize = match resp.content_length() {
            Some(ts) => ts,
            None => 0,
        };

        let mut stream = resp.bytes_stream();
        let mut downloadedsize = 0;
        while let Some(chunk_result) = stream.next().await {
            let chunk = chunk_result?;
            downloadedsize += chunk.len() as u64;
            save_file.write_all(&chunk).await?;
            receiver.try_send(MSTaskStatus::new(
                self.name.clone(),
                totalsize,
                downloadedsize,
                false,
            ))?;
        }
        save_file.flush().await?;
        match receiver
            .send(MSTaskStatus::new(
                self.name.clone(),
                totalsize,
                downloadedsize,
                true,
            ))
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::MSTaskMPSC),
        }
    }
}

pub struct DeleteTask {
    path: Option<PathBuf>,
    name: String,
}

impl DeleteTask {
    pub fn build(name: String, path: PathBuf) -> DeleteTask {
        DeleteTask {
            path: Some(path),
            name,
        }
    }
}

#[async_trait]
impl MSTask for DeleteTask {
    async fn start(&mut self, receiver: Sender<MSTaskStatus>) -> Result<(), Error> {
        let path = self.path.take().unwrap();
        std::fs::remove_file(path)?;
        match receiver
            .send(MSTaskStatus::new(self.name.clone(), 1, 1, true))
            .await
        {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::MSTaskMPSC),
        }
    }
}
