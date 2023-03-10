mod api;
mod responses;

use color_eyre::{eyre::eyre, Result};

pub use self::responses::SeriesStatus;
use self::responses::{Response, SeriesResource};

pub async fn get_sonarr_data(tvdb_id: u32) -> Result<SeriesResource> {
    let tvdb_string = tvdb_id.to_string();
    let params = vec![("tvdbId", tvdb_string.as_str())];
    let mut response: Response<SeriesResource> = api::get("/series", Some(params)).await?;

    if let Some(resource) = response.pop_front() {
        Ok(resource)
    } else {
        Err(eyre!("Got no response for tvdb id {}", tvdb_id))
    }
}

pub async fn remove_sonarr_data_and_files(sonarr_id: i32) -> Result<()> {
    let path = format!("/series/{}", sonarr_id.to_string());
    let params = vec![("deleteFiles", "true"), ("addImportListExclusion", "false")];
    api::delete(path.as_str(), Some(params)).await
}
