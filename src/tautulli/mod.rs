mod api;
mod responses;

use chrono::prelude::*;
use color_eyre::Result;

use crate::{shared::MediaType, tautulli::responses::ResponseObj};

use self::responses::{HistoryMovie, HistoryTv};

pub struct WatchHistory {
    pub rating_key: String,
    pub media_type: String,
    pub last_watched_history: Vec<UserLastWatch>,
}

pub struct UserLastWatch {
    pub display_name: String,
    pub last_watched: DateTime<Local>,
    pub progress: u32,
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

enum ItemType {
    Movie(ResponseObj<HistoryMovie>),
    Tv(ResponseObj<HistoryTv>),
}

pub async fn get_item_watches(rating_key: &str, media_type: &MediaType) -> Result<()> {
    let params = match media_type {
        MediaType::Movie => vec![("rating_key".to_string(), rating_key.to_string())],
        MediaType::Tv => vec![("grandparent_rating_key".to_string(), rating_key.to_string())],
    };

    let response_data = match media_type {
        MediaType::Movie => ItemType::Movie(api::getObj("get_history", Some(params)).await?),
        MediaType::Tv => ItemType::Tv(api::getObj("get_history", Some(params)).await?),
    };

    match response_data {
        ItemType::Movie(response) => println!("{:?}", response),
        ItemType::Tv(response) => println!("{:?}", response),
    }

    Ok(())
}
