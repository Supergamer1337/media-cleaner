use color_eyre::{eyre::eyre, owo_colors::OwoColorize, Report, Result};
use futures::future::{self};
use itertools::Itertools;
use std::fmt::{Debug, Display};
use tokio::try_join;

use crate::{
    arr::{self, ArrData},
    overseerr::{MediaRequest, MediaStatus},
    plex::PlexData,
    shared::MediaType,
    tautulli::{self, WatchHistory},
};

#[derive(Debug)]
pub struct MediaItem {
    pub title: Option<String>,
    rating_key: Option<String>,
    manager_id: Option<i32>,
    pub media_type: MediaType,
    request: MediaRequest,
}

impl MediaItem {
    pub fn from_request(request: MediaRequest) -> Self {
        Self {
            title: None,
            rating_key: request.rating_key.clone(),
            manager_id: request.manager_id,
            media_type: request.media_type,
            request: request,
        }
    }

    pub async fn into_complete(self) -> Result<CompleteMediaItem> {
        let metadata = self.retrieve_metadata();
        let history = self.retrieve_history();
        let data = self.retrieve_arr_data();
        let res = try_join!(metadata, history, data)?;

        let (details, history, arr_data) = res;
        Ok(CompleteMediaItem {
            title: details.title.clone(),
            media_type: self.media_type,
            request: self.request,
            history,
            arr_data,
        })
    }

    pub fn is_available(&self) -> bool {
        match &self.request.media_status {
            MediaStatus::Available | MediaStatus::PartiallyAvailable => true,
            _ => false,
        }
    }

    pub fn has_manager_active(&self) -> bool {
        match &self.request.media_type {
            MediaType::Movie => arr::movie_manger_active(),
            MediaType::Tv => arr::tv_manager_active(),
        }
    }

    async fn retrieve_history(&self) -> Result<WatchHistory> {
        let rating_key = match self.rating_key {
            Some(ref rating_key) => rating_key,
            None => {
                return Err(eyre!(
                "No rating key was found for request. Unable to gather watch history from Tautulli."
            ))
            }
        };

        tautulli::get_item_watches(rating_key, &self.media_type).await
    }

    async fn retrieve_metadata(&self) -> Result<PlexData> {
        let rating_key = match self.rating_key {
            Some(ref rating_key) => rating_key,
            None => {
                return Err(eyre!(
                    "No rating key was found for request. Unable to gather metadata from Plex."
                ))
            }
        };

        PlexData::get_data(rating_key, self.media_type).await
    }

    async fn retrieve_arr_data(&self) -> Result<ArrData> {
        if let Some(id) = self.manager_id {
            ArrData::get_data(self.media_type, id).await
        } else {
            Err(eyre!(
                "No *arr id was found for request. Unable to gather file data."
            ))
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

    pub fn get_disk_size(&self) -> i64 {
        self.arr_data.get_disk_size()
    }
}

pub async fn gather_requests_data() -> Result<(Vec<CompleteMediaItem>, Vec<Report>)> {
    let requests = MediaRequest::get_all().await?;

    let futures = requests
        .into_iter()
        .map(MediaItem::from_request)
        .filter(|i| i.is_available() && i.has_manager_active())
        .map(|item| {
            tokio::spawn(async move {
                let item = item.into_complete().await?;

                Ok::<CompleteMediaItem, Report>(item)
            })
        });

    let mut errors: Vec<Report> = Vec::new();

    let complete_items = future::try_join_all(futures)
        .await?
        .into_iter()
        .filter_map(|f| match f {
            Ok(item) => Some(item),
            Err(err) => {
                errors.push(err);
                None
            }
        })
        .unique_by(|item| item.title.clone())
        .sorted_by(|item1, item2| item1.title.cmp(&item2.title))
        .collect();

    Ok((complete_items, errors))
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
