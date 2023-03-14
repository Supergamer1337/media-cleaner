use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovieResource {
    pub id: i32,
    pub title: Option<String>,
    pub status: MovieStatus,
    pub size_on_disk: i64,
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
