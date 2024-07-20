use crate::error::Error;

use std::io::Cursor;

pub struct HttpGetResponse {
    pub code: u16,
    pub text: String,
}

pub async fn http_get(url: &str) -> Result<HttpGetResponse, Error> {
    let resp = reqwest::get(url).await?;
    Ok(HttpGetResponse {
        code: resp.status().as_u16(),
        text: resp.text().await?,
    })
}

pub async fn http_download(url: &str, filename: &str) -> Result<(), Error> {
    let resp = reqwest::get(url).await?;
    let mut file = std::fs::File::create(filename)?;
    let mut content = Cursor::new(resp.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}
