use serde::Deserialize;

pub type Response<T> = Vec<T>;

#[derive(Debug, Deserialize)]
pub struct MovieResource {
    pub id: i32,
    pub title: Option<String>,
    pub status: MovieStatus,
    pub size_on_disk: Option<i64>,
    pub digital_release: Option<String>,
    pub physical_release: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MovieStatus {
    #[serde(rename = "tba")]
    ToBeAnnounced,
    Announced,
    InCinemas,
    Released,
    Deleted,
}
