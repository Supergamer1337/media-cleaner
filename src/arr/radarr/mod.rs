mod api;
mod responses;

use color_eyre::{eyre::eyre, Result};

pub use self::responses::MovieStatus;
use self::responses::{MovieResource, Response};

pub async fn get_radarr_data(tmdb_id: u32) -> Result<MovieResource> {
    let tmdb_string = tmdb_id.to_string();
    let params = vec![("tmdbId", tmdb_string.as_str())];
    let mut response: Response<MovieResource> = api::get("/movie", Some(params)).await?;

    if let Some(resource) = response.pop_front() {
        Ok(resource)
    } else {
        Err(eyre!("Got no response for tmdb id {}", tmdb_id))
    }
}

pub async fn delete_radarr_data_and_files(radarr_id: i32) -> Result<()> {
    let path = format!("/movie/{}", radarr_id.to_string());
    let params = vec![("deleteFiles", "true"), ("addImportExclusion", "false")];
    api::delete(path.as_str(), Some(params)).await
}
