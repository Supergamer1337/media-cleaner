use serde::{de::DeserializeOwned, Deserialize};

pub type Response<T>
where
    T: DeserializeOwned,
= Vec<T>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesResource {
    pub id: i32,
    pub title: Option<String>,
    pub status: SeriesStatus,
    pub previous_airing: Option<String>,
    pub next_airing: Option<String>,
    pub statistics: SeriesStatisticsResource,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum SeriesStatus {
    Continuing,
    Ended,
    Upcoming,
    Deleted,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SeriesStatisticsResource {
    pub season_count: i32,
    pub episode_file_count: i32,
    pub episode_count: i32,
    pub size_on_disk: i64,
    pub percent_of_episodes: f64,
}
