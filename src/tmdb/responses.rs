use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct MovieDetails {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct TvDetails {
    pub name: String,
}
