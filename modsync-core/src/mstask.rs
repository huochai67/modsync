use crate::error::Error;
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
    fn get_join_handle(&self) -> &JoinHandle<Result<(), Error>>;

    async fn spawn(&mut self) -> Result<(), Error>;
}

pub struct DownloadTask {
    reqclient: reqwest::Client,
    totalsize: u64,
    downloadedsize: Arc<Mutex<u64>>,
    joinhandle: Option<JoinHandle<Result<(), Error>>>,
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
    fn get_join_handle(&self) -> &JoinHandle<Result<(), Error>> {
        self.joinhandle.as_ref().unwrap()
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    async fn spawn(&mut self) -> Result<(), Error> {
        let path = Path::new(self.savepath.as_str()).parent().unwrap();
        tokio::fs::create_dir_all(path).await?;

        let mut save_file = File::create(self.savepath.as_str()).await?;

        let resp = self.reqclient.get(self.url.as_str()).send().await?;
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
    joinhandle: Option<JoinHandle<Result<(), Error>>>,
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
    #[allow(unreachable_code)]
    async fn get_size_downloaded(&self) -> u64 {
        !todo!()
    }

    #[allow(unreachable_code)]
    fn get_size_total(&self) -> u64 {
        !todo!()
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_join_handle(&self) -> &JoinHandle<Result<(), Error>> {
        self.joinhandle.as_ref().unwrap()
    }

    async fn spawn(&mut self) -> Result<(), Error> {
        let path = self.path.take().unwrap();
        self.joinhandle = Some(tokio::spawn(async move { Ok(std::fs::remove_file(path)?) }));
        Ok(())
    }
}
