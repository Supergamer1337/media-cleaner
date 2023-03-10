mod radarr;
mod sonarr;

use chrono::{DateTime, Utc};
use color_eyre::Result;

pub use self::radarr::MovieStatus;
pub use self::sonarr::SeriesStatus;
use crate::shared::MediaType;

#[derive(Debug)]
pub enum ArrData {
    Movie(MovieData),
    Tv(TvData),
}

impl ArrData {
    pub async fn get_data(media_type: MediaType, tv_or_tmdb_id: u32) -> Result<Self> {
        match media_type {
            MediaType::Movie => Ok(Self::Movie(MovieData::get_data(tv_or_tmdb_id).await?)),
            MediaType::Tv => Ok(Self::Tv(TvData::get_data(tv_or_tmdb_id).await?)),
        }
    }

    pub async fn remove_data(self) -> Result<()> {
        match self {
            Self::Movie(movie) => movie.remove_data().await,
            Self::Tv(tv) => tv.remove_data().await,
        }
    }
}

#[derive(Debug)]
pub struct MovieData {
    id: i32,
    title: Option<String>,
    status: MovieStatus,
    size_on_disk: Option<i64>,
    digital_release: Option<DateTime<Utc>>,
    physical_release: Option<DateTime<Utc>>,
}

impl MovieData {
    async fn get_data(tmdb_id: u32) -> Result<Self> {
        let data = radarr::get_radarr_data(tmdb_id).await?;

        Ok(Self {
            id: data.id,
            title: data.title,
            status: data.status,
            size_on_disk: data.size_on_disk,
            digital_release: get_potential_date_time(data.digital_release)?,
            physical_release: get_potential_date_time(data.physical_release)?,
        })
    }

    async fn remove_data(self) -> Result<()> {
        radarr::delete_radarr_data_and_files(self.id).await
    }
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
}

impl TvData {
    async fn remove_data(self) -> Result<()> {
        sonarr::remove_sonarr_data_and_files(self.id).await
    }

    async fn get_data(tvdb_id: u32) -> Result<Self> {
        let data = sonarr::get_sonarr_data(tvdb_id).await?;

        Ok(Self {
            id: data.id,
            title: data.title,
            last_airing: get_potential_date_time(data.previous_airing)?,
            next_airing: get_potential_date_time(data.next_airing)?,
            status: data.status,
            season_count: data.statistics.season_count,
            percent_of_episodes_on_disk: data.statistics.percent_of_episodes,
            size_on_disk: data.statistics.size_on_disk,
        })
    }
}

fn get_potential_date_time(potential_date: Option<String>) -> Result<Option<DateTime<Utc>>> {
    match potential_date {
        Some(ref date) => {
            let date = DateTime::parse_from_rfc3339(date)?;
            Ok(Some(date.with_timezone(&Utc)))
        }
        None => Ok(None),
    }
}
