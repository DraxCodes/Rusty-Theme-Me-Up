use reqwest::header;
use serde::Deserialize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://wallhaven.cc/api/v1/search")
        .header(header::USER_AGENT, "Mozilla/5.0 (Windows NT 6.1; Win64; x64; rv:47.0) Gecko/20100101 Firefox/47.0")
        .send()
        .await?;
    
    let results: WallhavenResult = res.json().await?;
    
    println!("{:#?}", results);

    Ok(())
}

#[derive(Deserialize, Debug)]
struct WallhavenResult {
    data: Vec<WallpaperResult>,
    meta: Meta
}

#[derive(Deserialize, Debug)]
struct WallpaperResult {
    id: String,
    url: String,
    short_url: String,
    views: i32,
    favorites: i32,
    source: String,
    purity: String,
    category: String,
    dimension_x: i32,
    dimension_y: i32,
    resolution: String,
    ratio: String,
    file_size: i32,
    file_type: String,
    created_at: String,
    colors: Vec<String>,
    path: String,
    thumbs: Thumbs,
}

#[derive(Deserialize, Debug)]
struct Thumbs {
    large: String,
    original: String,
    small: String,
}

#[derive(Deserialize, Debug)]
struct Meta {
    current_page: i32,
    last_page: i32,
    per_page: i32,
    total: i32,
}