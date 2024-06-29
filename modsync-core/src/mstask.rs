use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use async_trait::async_trait;
use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex, task::JoinHandle};

#[async_trait]
pub trait MSTask {
    async fn get_size_downloaded(&self) -> u64;
    fn get_size_total(&self) -> u64;
    fn get_name(&self) -> &str;
    fn get_join_handle(&self) -> &JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;

    async fn spawn(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct DownloadTask {
    totalsize: u64,
    downloadedsize: Arc<Mutex<u64>>,
    joinhandle: Option<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>>,
    name: String,
    url: String,
    savepath: String,
}

impl DownloadTask {
    pub fn build(name: String, url: String, savepath: String) -> DownloadTask {
        DownloadTask {
            totalsize: 0,
            downloadedsize: Arc::from(Mutex::new(0)),
            name,
            url,
            savepath,
            joinhandle: None,
        }
    }
}

#[async_trait]
impl MSTask for DownloadTask {
    async fn get_size_downloaded(&self) -> u64 {
        self.downloadedsize.lock().await.clone()
    }
    fn get_size_total(&self) -> u64 {
        self.totalsize.clone()
    }
    fn get_join_handle(&self) -> &JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        self.joinhandle.as_ref().unwrap()
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    async fn spawn(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new(self.savepath.as_str()).parent().unwrap();
        tokio::fs::create_dir_all(path).await?;

        let mut save_file = File::create(self.savepath.as_str()).await?;

        let resp = reqwest::get(self.url.as_str()).await?;
        self.totalsize = match resp.content_length() {
            Some(ts) => ts,
            None => 0,
        };

        let mut stream = resp.bytes_stream();
        let ptr_size = self.downloadedsize.clone();

        self.joinhandle = Some(tokio::spawn(async move {
            while let Some(chunk_result) = stream.next().await {
                let chunk = chunk_result?;
                *ptr_size.lock().await += chunk.len() as u64;
                save_file.write_all(&chunk).await?;
            }
            save_file.flush().await?;
            Ok(())
        }));
        Ok(())
    }
}

pub struct DeleteTask {
    path: Option<PathBuf>,
    name: String,
    joinhandle: Option<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>>,
}

impl DeleteTask {
    pub fn build(name: String, path: PathBuf) -> DeleteTask {
        DeleteTask {
            path: Some(path),
            name,
            joinhandle: None,
        }
    }
}

#[async_trait]
impl MSTask for DeleteTask {
    async fn get_size_downloaded(&self) -> u64 {
        1
    }

    fn get_size_total(&self) -> u64 {
        1
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_join_handle(&self) -> &JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
        self.joinhandle.as_ref().unwrap()
    }

    async fn spawn(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let path = self.path.take().unwrap();
        self.joinhandle = Some(tokio::spawn(async move {
            match std::fs::remove_file(path) {
                Ok(_) => Ok(()),
                Err(err) => Err(Box::from(err)),
            }
        }));
        Ok(())
    }
}
