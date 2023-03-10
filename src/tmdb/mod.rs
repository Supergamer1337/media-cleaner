mod api;
mod responses;

use crate::shared::MediaType;
use color_eyre::Result;

#[derive(Debug)]
pub struct ItemMetadata {
    pub name: String,
}

impl ItemMetadata {
    pub async fn get_data(media_type: MediaType, tmdb_id: u32) -> Result<Self> {
        match media_type {
            MediaType::Movie => {
                let path = &format!("/movie/{}", tmdb_id);
                let result: responses::MovieDetails = api::get(path, None).await?;

                Ok(ItemMetadata { name: result.title })
            }
            MediaType::Tv => {
                let path = &format!("/tv/{}", tmdb_id);
                let result: responses::TvDetails = api::get(path, None).await?;

                Ok(ItemMetadata { name: result.name })
            }
        }
    }
}
