mod radarr;
mod sonarr;

use std::fmt::Display;

use chrono::{DateTime, Utc};
use color_eyre::owo_colors::OwoColorize;
use color_eyre::Result;

pub use self::radarr::MovieResource;
pub use self::sonarr::SeriesResource;
use crate::config::Config;

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

pub async fn get_all_items() -> Result<Vec<ArrData>> {
    let mut media_items = vec![];
    if tv_manager_active() {
        media_items.extend(
            sonarr::get_all_items(false)
                .await?
                .into_iter()
                .map(|res| ArrData::Tv(res)),
        );

        if tv_4k_manager_active() {
            media_items.extend(
                sonarr::get_all_items(true)
                    .await?
                    .into_iter()
                    .map(|res| ArrData::Tv(res)),
            );
        }
    }

    if movie_manger_active() {
        media_items.extend(
            radarr::get_all_items(false)
                .await?
                .into_iter()
                .map(|res| ArrData::Movie(res)),
        );

        if movie_4k_manager_active() {
            media_items.extend(
                radarr::get_all_items(true)
                    .await?
                    .into_iter()
                    .map(|res| ArrData::Movie(res)),
            );
        }
    }

    Ok(media_items)
}

#[derive(Debug)]
pub enum ArrData {
    Movie(MovieResource),
    Tv(SeriesResource),
}

impl ArrData {
    pub fn get_disk_size(&self) -> i64 {
        match self {
            Self::Movie(movie) => movie.size_on_disk,
            Self::Tv(tv) => tv.statistics.size_on_disk,
        }
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
