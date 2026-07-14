use crate::error::Error;

use futures_util::StreamExt;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub struct HttpGetResponse {
    pub code: u16,
    pub text: String,
}

pub async fn http_get(url: &str) -> Result<HttpGetResponse, Error> {
    let resp = reqwest::get(url).await?.error_for_status()?;
    Ok(HttpGetResponse {
        code: resp.status().as_u16(),
        text: resp.text().await?,
    })
}

pub async fn http_download(url: &str, filename: &str) -> Result<(), Error> {
    let response = reqwest::get(url).await?.error_for_status()?;
    let target = Path::new(filename);
    let parent = target
        .parent()
        .ok_or_else(|| Error::Validation("download target has no parent directory".to_string()))?;
    tokio::fs::create_dir_all(parent).await?;

    let temporary = target.with_extension(format!(
        "{}.part",
        target
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or("download")
    ));
    let mut file = tokio::fs::File::create(&temporary).await?;
    let mut stream = response.bytes_stream();
    let result: Result<(), Error> = async {
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk?).await?;
        }
        file.flush().await?;
        Ok(())
    }
    .await;
    drop(file);

    if let Err(error) = result {
        let _ = tokio::fs::remove_file(&temporary).await;
        return Err(error);
    }
    if let Err(error) = tokio::fs::rename(&temporary, target).await {
        let _ = tokio::fs::remove_file(&temporary).await;
        return Err(error.into());
    }
    Ok(())
}
