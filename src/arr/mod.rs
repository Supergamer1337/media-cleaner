mod radarr;
mod sonarr;

use chrono::{DateTime, Utc};
use color_eyre::Result;

pub use self::radarr::MovieStatus;
pub use self::sonarr::SeriesStatus;

pub struct MovieData {
    pub id: String,
    pub title: Option<String>,
    pub status: MovieStatus,
    pub size_on_disk: Option<i64>,
    pub digital_release: Option<DateTime<Utc>>,
    pub physical_release: Option<DateTime<Utc>>,
    pub is_available: bool,
}

pub async fn get_movie_data(tmdb_id: u32) -> Result<MovieData> {
    Ok(radarr::get_radarr_data(tmdb_id).await?)
}

#[derive(Debug)]
pub struct TvData {
    id: i32,
    title: Option<String>,
    status: SeriesStatus,
    last_airing: Option<DateTime<Utc>>,
    next_airing: Option<DateTime<Utc>>,
    season_count: i32,
    percent_of_episodes_on_disk: f64,
    size_on_disk: i64,
    is_available: bool,
}

pub async fn get_tv_data(tvdb_id: u32) -> Result<TvData> {
    Ok(sonarr::get_sonarr_data(tvdb_id).await?)
}

fn date_string_to_date_time(date: &str) -> Result<DateTime<Utc>> {
    let date = DateTime::parse_from_rfc3339(date)?;
    Ok(date.with_timezone(&Utc))
}
