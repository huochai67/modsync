use std::{path::PathBuf, sync::Arc};

use futures::StreamExt;
use tokio::{fs::File, io::AsyncWriteExt, sync::Mutex, task::JoinHandle};

pub trait MSTask {
    async fn get_size_downloaded(&self) -> u64;
    fn get_size_total(&self) -> u64;
    fn get_name(&self) -> &str;
    fn get_join_handle(&self) -> &JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;
}

pub struct DownloadTask {
    totalsize: u64,
    downloadedsize: Arc<Mutex<u64>>,
    joinhandle: Option<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>>,
    name: String,
    url: String,
    savepath : String,
}

impl DownloadTask {
    pub fn build(
        name: String,
        url: String,
        savepath : String,
    ) -> DownloadTask {
        DownloadTask {
            totalsize: 0,
            downloadedsize: Arc::from(Mutex::new(0)),
            name,
            url,
            savepath,
            joinhandle: None,
        }
    }

    pub async fn spawn(&mut self) -> Result<(), Box<dyn std::error::Error>> {
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
}

pub struct DeleteTask {
    path: Option<PathBuf>,
    name: String,
    joinhandle: Option<JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>>,
}

impl DeleteTask {
    pub fn build(name: String, path: PathBuf) -> Result<DeleteTask, Box<dyn std::error::Error>> {
        Ok(DeleteTask {
            path: Some(path),
            name,
            joinhandle: None,
        })
    }

    fn spawn(&mut self) {
        let path = self.path.take().unwrap();
        let jg = tokio::spawn(async move {
            match std::fs::remove_file(path) {
                Ok(_) => Ok(()),
                Err(_) => Err(Box::from("123")),
            }
        });
        self.joinhandle = Some(jg);
    }
}

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
}
