mod radarr;
mod sonarr;

use chrono::{DateTime, Utc};
use color_eyre::Result;

pub use self::radarr::MovieStatus;
use self::sonarr::remove_sonarr_data_and_files;
pub use self::sonarr::SeriesStatus;

#[derive(Debug)]
pub struct MovieData {
    pub id: i32,
    pub title: Option<String>,
    pub status: MovieStatus,
    pub size_on_disk: Option<i64>,
    pub digital_release: Option<DateTime<Utc>>,
    pub physical_release: Option<DateTime<Utc>>,
}

pub async fn get_movie_data(tmdb_id: u32) -> Result<MovieData> {
    Ok(radarr::get_radarr_data(tmdb_id).await?)
}

#[derive(Debug)]
pub struct TvData {
    pub id: i32,
    pub title: Option<String>,
    pub status: SeriesStatus,
    pub last_airing: Option<DateTime<Utc>>,
    pub next_airing: Option<DateTime<Utc>>,
    pub season_count: i32,
    pub percent_of_episodes_on_disk: f64,
    pub size_on_disk: i64,
}

pub async fn get_tv_data(tvdb_id: u32) -> Result<TvData> {
    Ok(sonarr::get_sonarr_data(tvdb_id).await?)
}

fn date_string_to_date_time(date: &str) -> Result<DateTime<Utc>> {
    let date = DateTime::parse_from_rfc3339(date)?;
    Ok(date.with_timezone(&Utc))
}

pub async fn remove_tv_data(id: i32) -> Result<()> {
    sonarr::remove_sonarr_data_and_files(id).await
}

pub async fn remove_movie_data(id: i32) -> Result<()> {
    radarr::delete_radarr_data_and_files(id).await
}
