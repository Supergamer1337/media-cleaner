mod api;
mod responses;

use self::responses::TvData;
use crate::{plex::responses::MovieData, shared::MediaType};
use color_eyre::Result;
use itertools::Itertools;

#[derive(Debug)]
pub struct PlexData {
    pub rating_key: String,
    pub tvdb_id: Option<String>,
    pub tmdb_id: Option<String>,
}

pub async fn get_all_items() -> Result<Vec<PlexData>> {
    let series: responses::TvData = api::get(
        "/library/all",
        Some(vec![("type", "2"), ("includeGuids", "1")]),
    )
    .await?;

    let movies: responses::MovieData = api::get(
        "/library/all",
        Some(vec![("type", "1"), ("includeGuids", "1")]),
    )
    .await?;

    let mut plex_data = Vec::<PlexData>::from(&series);
    plex_data.extend(Vec::<PlexData>::from(&movies));

    Ok(plex_data)
}

impl PlexData {
    fn parse_guids(guids: &[responses::Guid]) -> (Option<String>, Option<String>) {
        guids.iter().fold((None, None), |(tvdb, tmdb), guid| {
            match guid.id.split("://").collect_tuple() {
                Some(("tvdb", id)) => (Some(id.to_owned()), tmdb),
                Some(("tmdb", id)) => (tvdb, Some(id.to_owned())),
                _ => (tvdb, tmdb),
            }
        })
    }

    fn from_guids(rating_key: String, guids: &[responses::Guid]) -> PlexData {
        let (tvdb_id, tmdb_id) = Self::parse_guids(guids);
        PlexData {
            rating_key,
            tvdb_id,
            tmdb_id,
        }
    }
}

impl From<&TvData> for Vec<PlexData> {
    fn from(data: &TvData) -> Self {
        data.directory
            .iter()
            .map(|show| PlexData::from_guids(show.rating_key.clone(), &show.guids))
            .collect()
    }
}

impl From<&MovieData> for Vec<PlexData> {
    fn from(data: &MovieData) -> Self {
        data.video
            .iter()
            .map(|video| PlexData::from_guids(video.rating_key.clone(), &video.guids))
            .collect()
    }
}
