mod api;
mod responses;

use chrono::{DateTime, Utc};
use color_eyre::{eyre::eyre, Result};

pub use self::responses::SeriesStatus;
use crate::sonarr::responses::{Response, SeriesResource};

#[derive(Debug)]
pub struct SonarrData {
    id: i32,
    title: Option<String>,
    status: SeriesStatus,
    last_airing: Option<DateTime<Utc>>,
    next_airing: Option<DateTime<Utc>>,
    season_count: i32,
    percent_of_episodes_on_disk: f64,
    size_on_disk: i64,
}

pub async fn get_sonarr_data(tvdb_id: u32) -> Result<SonarrData> {
    let tvdb_string = tvdb_id.to_string();
    let params = vec![("tvdbId", tvdb_string.as_str())];
    let response: Response<SeriesResource> = api::get("/series", Some(params)).await?;

    if let Some(response) = response.get(0) {
        Ok(SonarrData {
            id: response.id,
            title: response.title.clone(),
            status: response.status,
            last_airing: match response.previous_airing {
                Some(ref date) => Some(date_string_to_date_time(date)?),
                None => None,
            },
            next_airing: match response.next_airing {
                Some(ref date) => Some(date_string_to_date_time(date)?),
                None => None,
            },
            season_count: response.statistics.season_count,
            percent_of_episodes_on_disk: response.statistics.percent_of_episodes,
            size_on_disk: response.statistics.size_on_disk,
        })
    } else {
        Err(eyre!("Got no response for tvdb id {}", tvdb_id))
    }
}

fn date_string_to_date_time(date: &str) -> Result<DateTime<Utc>> {
    let date = DateTime::parse_from_rfc3339(date)?;
    Ok(date.with_timezone(&Utc))
}
