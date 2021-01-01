use std::collections::HashMap;
use reqwest::header;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://wallhaven.cc/api/v1/search")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0")
        .send()
        .await?;
    
    println!("{:#?}", res);

    Ok(())
}
