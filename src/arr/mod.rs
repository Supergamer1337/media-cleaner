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

pub fn tv_manager_active() -> bool {
    match Config::global().sonarr {
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
    size_on_disk: Option<i64>,
    digital_release: Option<DateTime<Utc>>,
    physical_release: Option<DateTime<Utc>>,
}

impl MovieData {
    async fn get_data(tmdb_id: u32) -> Result<Self> {
        let data = radarr::get_radarr_data(tmdb_id).await?;

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

        let size: String = match self.size_on_disk {
            Some(size) => human_file_size(size),
            None => "none".into(),
        };

        write!(
            f,
            "It was released {} digitally and {} physically. Current status is {:?}, and size is {}",
            digital_release.blue(),
            physical_release.blue(),
            self.status.green(),
            size.red()
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
            last_airing: get_potential_date_time(data.previous_airing)?,
            next_airing: get_potential_date_time(data.next_airing)?,
            status: data.status,
            season_count: data.statistics.season_count,
            percent_of_episodes_on_disk: data.statistics.percent_of_episodes,
            size_on_disk: data.statistics.size_on_disk,
        })
    }
}

impl Display for TvData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let size: String = human_file_size(self.size_on_disk);
        let last_aired = format_potential_date(self.last_airing);
        let next_airing = format_potential_date(self.next_airing);

        write!(
            f,
            "Last airing was {} and the next {}. Current status is {:?}, and size is {}. It has {} seasons, with {} of episodes downloaded.",
            last_aired.blue(),
            next_airing.blue(),
            self.status.green(),
            size.red(),
            self.season_count.yellow(),
            &format!("{}%", self.percent_of_episodes_on_disk).blue()
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

fn human_file_size(size: i64) -> String {
    let gig_size = 1000000000.0;
    let gigs: f64 = size as f64 / gig_size;
    format!("{:.2}GB", gigs)
}
