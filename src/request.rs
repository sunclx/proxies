use anyhow::Result;
use reqwest;

pub async fn get_config(url: &str) -> Result<String> {
    let content = reqwest::get(url).await?.text().await?;
    return Ok(content);
}
