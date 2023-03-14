mod api;
mod responses;

use crate::{plex::responses::MovieData, shared::MediaType};

use self::responses::TvData;

use color_eyre::Result;

pub struct PlexData {
    pub title: String,
}

impl PlexData {
    pub async fn get_data(rating_key: &str, media_type: MediaType) -> Result<Self> {
        println!("Getting Plex data for {}", rating_key);
        let path = format!("/library/metadata/{}", rating_key);
        match media_type {
            MediaType::Movie => {
                let raw_plex_data: MovieData = api::get(&path, None).await?;
                println!("Done getting Plex data for {}", rating_key);

                Ok(Self {
                    title: raw_plex_data.video.title,
                })
            }
            MediaType::Tv => {
                let raw_plex_data: TvData = api::get(&path, None).await?;
                println!("Done getting Plex data for {}", rating_key);

                Ok(Self {
                    title: raw_plex_data.directory.title,
                })
            }
        }
    }
}
