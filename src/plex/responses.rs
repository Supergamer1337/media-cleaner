use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TvData {
    pub directory: Vec<Show>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    pub title: String,
    pub rating_key: String,
    #[serde(rename = "Guid", default)]
    pub guids: Vec<Guid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MovieData {
    pub video: Vec<Video>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub title: String,
    pub rating_key: String,
    #[serde(rename = "Guid", default)]
    pub guids: Vec<Guid>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guid {
    pub id: String,
}
