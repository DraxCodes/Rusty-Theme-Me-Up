use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct WallhavenResult {
    pub data: Vec<WallpaperResult>,
    pub meta: Meta,
}

#[derive(Deserialize, Debug)]
pub struct WallpaperResult {
    pub id: String,
    pub url: String,
    pub short_url: String,
    pub views: i32,
    pub favorites: i32,
    pub source: String,
    pub purity: String,
    pub category: String,
    pub dimension_x: i32,
    pub dimension_y: i32,
    pub resolution: String,
    pub ratio: String,
    pub file_size: i32,
    pub file_type: String,
    pub created_at: String,
    pub colors: Vec<String>,
    pub path: String,
    pub thumbs: Thumbs,
}

#[derive(Deserialize, Debug)]
pub struct Thumbs {
    pub large: String,
    pub original: String,
    pub small: String,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub current_page: i32,
    pub last_page: i32,
    pub per_page: i32,
    pub total: i32,
}
