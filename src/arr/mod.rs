mod radarr;
mod sonarr;

use std::fmt::Display;

use chrono::{DateTime, Utc};
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;

pub use self::radarr::MovieStatus;
pub use self::sonarr::SeriesStatus;
use crate::config::Config;
use crate::shared::MediaType;

pub fn movie_manger_active() -> bool {
    match Config::global().radarr {
        Some(_) => true,
        None => false,
    }
}

pub fn movie_4k_manager_active() -> bool {
    match Config::global().radarr_4k {
        Some(_) => true,
        None => false,
    }
}

pub fn tv_manager_active() -> bool {
    match Config::global().sonarr {
        Some(_) => true,
        None => false,
    }
}

pub fn tv_4k_manager_active() -> bool {
    match Config::global().sonarr_4k {
        Some(_) => true,
        None => false,
    }
}

#[derive(Debug)]
pub enum ArrData {
    Movie(MovieData),
    Tv(TvData),
}

impl ArrData {
    pub async fn get_data(media_type: MediaType, id: i32) -> Result<Self> {
        match media_type {
            MediaType::Movie => Ok(Self::Movie(MovieData::get_data(id, false).await?)),
            MediaType::Tv => Ok(Self::Tv(TvData::get_data(id, false).await?)),
        }
    }

    pub async fn get_4k_data(media_type: MediaType, id: i32) -> Result<Self> {
        match media_type {
            MediaType::Movie => Ok(Self::Movie(MovieData::get_data(id, true).await?)),
            MediaType::Tv => Ok(Self::Tv(TvData::get_data(id, true).await?)),
        }
    }

    pub async fn remove_data(self) -> Result<()> {
        match self {
            Self::Movie(movie) => movie.remove_data().await,
            Self::Tv(tv) => tv.remove_data().await,
        }
    }

    pub fn get_disk_size(&self) -> i64 {
        match self {
            Self::Movie(movie) => movie.size_on_disk,
            Self::Tv(tv) => tv.size_on_disk,
        }
    }
}

impl Display for ArrData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Movie(movie) => write!(f, "{}", movie),
            Self::Tv(tv) => write!(f, "{}", tv),
        }
    }
}

#[derive(Debug)]
pub struct MovieData {
    id: i32,
    status: MovieStatus,
    size_on_disk: i64,
    digital_release: Option<DateTime<Utc>>,
    physical_release: Option<DateTime<Utc>>,
}

impl MovieData {
    async fn get_data(id: i32, is_4k: bool) -> Result<Self> {
        let data = radarr::get_radarr_data(id, is_4k).await?;

        Ok(Self {
            id: data.id,
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

impl Display for MovieData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let digital_release = format_potential_date(self.digital_release);

        let physical_release = format_potential_date(self.physical_release);

        write!(
            f,
            "It was released {} digitally and {} physically. Current status is {:?}.",
            digital_release.blue(),
            physical_release.blue(),
            self.status.green(),
        )
    }
}

#[derive(Debug)]
pub struct TvData {
    id: i32,
    status: SeriesStatus,
    last_airing: Option<DateTime<Utc>>,
    next_airing: Option<DateTime<Utc>>,
    season_count: i32,
    episodes_in_last_season: i32,
    percent_of_episodes_on_disk: f64,
    size_on_disk: i64,
}

impl TvData {
    async fn remove_data(self) -> Result<()> {
        sonarr::remove_sonarr_data_and_files(self.id).await
    }

    async fn get_data(id: i32, is_4k: bool) -> Result<Self> {
        let data = sonarr::get_sonarr_data(id, is_4k).await?;

        let episodes_in_last_season = data
            .seasons
            .iter()
            .max_by_key(|s| s.season_number)
            .map(|s| s.statistics.episode_count);

        Ok(Self {
            id: data.id,
            last_airing: get_potential_date_time(data.previous_airing)?,
            next_airing: get_potential_date_time(data.next_airing)?,
            status: data.status,
            season_count: data.statistics.season_count,
            episodes_in_last_season: match episodes_in_last_season {
                Some(count) => count,
                None => 0,
            },
            percent_of_episodes_on_disk: data.statistics.percent_of_episodes,
            size_on_disk: data.statistics.size_on_disk,
        })
    }
}

impl Display for TvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let last_aired = format_potential_date(self.last_airing);
        let next_airing = format_potential_date(self.next_airing);

        write!(
            f,
            "Last airing was {} and the next {}. Current status is {:?}. It has {} seasons, and {} episodes in the last season, with {} of episodes downloaded.",
            last_aired.blue(),
            next_airing.blue(),
            self.status.green(),
            self.season_count.yellow(),
            match self.episodes_in_last_season {
                0 => "unknown".to_string(),
                count => count.to_string()
            }.yellow(),
            &format!("{:.2}%", self.percent_of_episodes_on_disk).blue(),
        )
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

fn format_potential_date(potential_date: Option<DateTime<Utc>>) -> String {
    match potential_date {
        Some(release) => release.format("%d-%m-%Y").to_string(),
        None => "never(?)".into(),
    }
}
