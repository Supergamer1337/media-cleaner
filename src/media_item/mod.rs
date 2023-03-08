mod constructors;
mod functionality;
mod getters_setters;
mod trait_impl;

use color_eyre::{Report, Result};
use futures::future;
use itertools::Itertools;
use std::cmp::Ordering;

use crate::{
    arr::{MovieData, TvData},
    overseerr::{self, MediaRequest},
    tautulli::WatchHistory,
    tmdb::ItemMetadata,
};

#[derive(Debug)]
pub enum MediaItem {
    Tv {
        title: Option<String>,
        rating_key: Option<String>,
        request: Option<MediaRequest>,
        history: Option<WatchHistory>,
        details: Option<ItemMetadata>,
        tv_data: Option<TvData>,
    },
    Movie {
        title: Option<String>,
        rating_key: Option<String>,
        request: Option<MediaRequest>,
        history: Option<WatchHistory>,
        details: Option<ItemMetadata>,
        movie_data: Option<MovieData>,
    },
}

pub async fn get_requests_data() -> Result<Vec<MediaItem>> {
    let requests = overseerr::get_requests().await.unwrap();

    let items = requests
        .into_iter()
        .map(|request| MediaItem::from_request(request))
        .filter(|item| item.is_request_available())
        .sorted_by(|item1, item2| {
            if let (Some(rating_key), Some(rating_key2)) =
                (item1.get_rating_key(), item2.get_rating_key())
            {
                rating_key.cmp(rating_key2)
            } else {
                Ordering::Less
            }
        })
        .dedup_by(|item1, item2| {
            if let (Some(rating_key), Some(rating_key2)) =
                (item1.get_rating_key(), item2.get_rating_key())
            {
                rating_key.eq(rating_key2)
            } else {
                false
            }
        })
        .collect_vec();

    let futures = items.into_iter().map(|mut item| {
        tokio::spawn(async move {
            item.get_all_info().await?;

            Ok::<MediaItem, Report>(item)
        })
    });

    Ok(future::try_join_all(futures)
        .await?
        .into_iter()
        .filter_map(|future| future.ok()) // TODO: Change this to give error messages to the user.
        .sorted_by(|item1, item2| {
            if let (Some(title), Some(title2)) = (item1.get_title(), item2.get_title()) {
                title.cmp(title2)
            } else {
                Ordering::Less
            }
        })
        .collect())
}
