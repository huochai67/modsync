use std::io::Cursor;

pub async fn http_get(url: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let resp = reqwest::get(url).await?;
    if resp.status() == 200 {
        return Ok(resp.text().await?);
    }
    Err(Box::from(format!("{} return {}", url, resp.status())))
}

pub async fn http_download(url: &str, filename : &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let resp = reqwest::get(url).await?;
    let mut file = std::fs::File::create(filename)?;
    let mut content =  Cursor::new(resp.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(())
}