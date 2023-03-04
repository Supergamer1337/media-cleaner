mod api;
mod responses;

use crate::shared::MediaType;
use color_eyre::Result;

#[derive(Debug)]
pub struct ItemMetadata {
    pub name: String,
}

pub async fn get_metadata(tmdb_id: u32, media_type: &MediaType) -> Result<ItemMetadata> {
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
