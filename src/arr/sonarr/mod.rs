mod api;
mod responses;

use color_eyre::{eyre::eyre, Result};

pub use self::responses::SeriesStatus;
use self::responses::{Response, SeriesResource};
use super::{date_string_to_date_time, TvData};

pub async fn get_sonarr_data(tvdb_id: u32) -> Result<TvData> {
    let tvdb_string = tvdb_id.to_string();
    let params = vec![("tvdbId", tvdb_string.as_str())];
    let response: Response<SeriesResource> = api::get("/series", Some(params)).await?;

    if let Some(response) = response.get(0) {
        TvData::from_series_resource(response)
    } else {
        Err(eyre!("Got no response for tvdb id {}", tvdb_id))
    }
}

impl TvData {
    fn from_series_resource(series_resource: &SeriesResource) -> Result<Self> {
        Ok(TvData {
            id: series_resource.id,
            title: series_resource.title.clone(),
            status: series_resource.status,
            last_airing: match series_resource.previous_airing {
                Some(ref date) => Some(date_string_to_date_time(date)?),
                None => None,
            },
            next_airing: match series_resource.next_airing {
                Some(ref date) => Some(date_string_to_date_time(date)?),
                None => None,
            },
            season_count: series_resource.statistics.season_count,
            percent_of_episodes_on_disk: series_resource.statistics.percent_of_episodes,
            size_on_disk: series_resource.statistics.size_on_disk,
        })
    }
}
