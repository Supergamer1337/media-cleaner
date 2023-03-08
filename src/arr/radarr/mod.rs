mod api;
mod responses;

use color_eyre::{eyre::eyre, Result};

pub use self::responses::MovieStatus;
use self::responses::{MovieResource, Response};
use super::{date_string_to_date_time, MovieData};

pub async fn get_radarr_data(tmdb_id: u32) -> Result<MovieData> {
    let tmdb_string = tmdb_id.to_string();
    let params = vec![("tmdbId", tmdb_string.as_str())];
    let response: Response<MovieResource> = api::get("/movie", Some(params)).await?;

    if let Some(response) = response.get(0) {
        MovieData::from_movie_resource(response)
    } else {
        Err(eyre!("Got no response for tmdb id {}", tmdb_id))
    }
}

impl MovieData {
    pub fn from_movie_resource(movie_resource: &MovieResource) -> Result<Self> {
        Ok(MovieData {
            id: movie_resource.id,
            title: movie_resource.title.clone(),
            status: movie_resource.status,
            size_on_disk: movie_resource.size_on_disk,
            digital_release: match movie_resource.digital_release {
                Some(ref date) => Some(date_string_to_date_time(date)?),
                None => None,
            },
            physical_release: match movie_resource.physical_release {
                Some(ref date) => Some(date_string_to_date_time(date)?),
                None => None,
            },
        })
    }
}
