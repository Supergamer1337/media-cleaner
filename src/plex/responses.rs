use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TvData {
    #[serde(rename = "Directory")]
    pub directory: Directory,
}

#[derive(Debug, Deserialize)]
pub struct Directory {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct MovieData {
    #[serde(rename = "Video")]
    pub video: Video,
}

#[derive(Debug, Deserialize)]
pub struct Video {
    pub title: String,
}
