use color_eyre::{owo_colors::OwoColorize, Report, Result};
use futures::future::{self};
use itertools::Itertools;
use std::fmt::{Debug, Display};
use tokio::try_join;

use crate::{
    arr::ArrData,
    overseerr::MediaRequest,
    shared::MediaType,
    tautulli::{self, WatchHistory},
    tmdb::ItemMetadata,
};

#[derive(Debug)]
pub struct MediaItem {
    pub title: Option<String>,
    rating_key: Option<String>,
    pub media_type: MediaType,
    request: MediaRequest,
}

impl MediaItem {
    pub fn from_request(request: MediaRequest) -> Self {
        Self {
            title: None,
            rating_key: request.rating_key.clone(),
            media_type: request.media_type,
            request: request,
        }
    }

    pub async fn into_complete(self) -> Result<Option<CompleteMediaItem>> {
        let metadata = self.retrieve_metadata();
        let history = self.retrieve_history();
        let data = self.retrieve_arr_data();
        let res = try_join!(metadata, history, data)?;

        if let (Some(details), Some(history), Some(arr_data)) = res {
            Ok(Some(CompleteMediaItem {
                title: details.name.clone(),
                media_type: self.media_type,
                request: self.request,
                history,
                arr_data,
            }))
        } else {
            Ok(None)
        }
    }

    async fn retrieve_history(&self) -> Result<Option<WatchHistory>> {
        let rating_key = match self.rating_key {
            Some(ref rating_key) => rating_key,
            None => return Ok(None),
        };

        Ok(Some(
            tautulli::get_item_watches(rating_key, &self.media_type).await?,
        ))
    }

    async fn retrieve_metadata(&self) -> Result<Option<ItemMetadata>> {
        let tmdb_id = match self.request.tmdb_id {
            Some(ref tmdb_id) => *tmdb_id,
            None => return Ok(None),
        };

        Ok(Some(
            ItemMetadata::get_data(self.media_type, tmdb_id).await?,
        ))
    }

    async fn retrieve_arr_data(&self) -> Result<Option<ArrData>> {
        let id = match self.media_type {
            MediaType::Movie => self.request.tmdb_id,
            MediaType::Tv => self.request.tvdb_id,
        };

        if let Some(id) = id {
            Ok(Some(ArrData::get_data(self.media_type, id).await?))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug)]
pub struct CompleteMediaItem {
    pub title: String,
    pub media_type: MediaType,
    request: MediaRequest,
    history: WatchHistory,
    arr_data: ArrData,
}

impl CompleteMediaItem {
    pub async fn remove_from_server(self) -> Result<()> {
        self.request.remove_request().await?;
        self.arr_data.remove_data().await?;
        Ok(())
    }
}

pub async fn get_requests_data() -> Result<Vec<CompleteMediaItem>> {
    let requests = MediaRequest::get_all().await?;

    let futures = requests
        .into_iter()
        .map(MediaItem::from_request)
        .map(|item| {
            tokio::spawn(async move {
                let item = item.into_complete().await?;

                Ok::<Option<CompleteMediaItem>, Report>(item)
            })
        });

    Ok(future::try_join_all(futures)
        .await?
        .into_iter()
        .filter_map(|f| f.ok()) // TODO: Change this to give error messages to the user.
        .filter_map(|f| f)
        .unique_by(|item| item.title.clone())
        .sorted_by(|item1, item2| item1.title.cmp(&item2.title))
        .collect())
}

impl Display for CompleteMediaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}.",
            self.media_type.to_string().blue(),
            self.title.green()
        )?;
        write!(f, " {}", self.request)?;

        write!(f, "\n      {}", self.arr_data)?;

        write!(f, "\n      {}", self.history)?;

        write!(f, "\n")
    }
}
