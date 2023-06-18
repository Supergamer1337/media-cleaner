use color_eyre::{eyre::eyre, owo_colors::OwoColorize, Result};
use std::fmt::{Debug, Display};
use tokio::try_join;

use crate::{
    arr::{self, ArrData},
    config::Config,
    overseerr::{MediaRequest, MediaStatus, ServerItem},
    plex::PlexData,
    shared::MediaType,
    tautulli::{self, WatchHistory},
    utils::human_file_size,
};

#[derive(Debug)]
pub struct MediaItem {
    pub title: Option<String>,
    pub rating_key: Option<String>,
    manager_id: Option<i32>,
    manager_4k_id: Option<i32>,
    pub media_type: MediaType,
    media_status: MediaStatus,
    pub request: Option<MediaRequest>,
}

impl MediaItem {
    pub fn from_request(request: MediaRequest) -> Self {
        Self {
            title: None,
            rating_key: request.rating_key.clone(),
            manager_id: request.manager_id,
            manager_4k_id: request.manager_4k_id,
            media_type: request.media_type,
            media_status: request.media_status,
            request: Some(request),
        }
    }

    pub fn from_server_item(item: ServerItem) -> Self {
        Self {
            title: None,
            rating_key: Some(item.rating_key),
            manager_id: item.manager_id,
            manager_4k_id: item.manager_id_4k,
            media_type: item.media_type,
            media_status: item.media_status,
            request: None,
        }
    }

    pub async fn into_complete_media(self) -> Result<CompleteMediaItem> {
        let metadata = self.retrieve_metadata();
        let history = self.retrieve_history();
        let data = self.retrieve_arr_data();

        let (details, history, (arr_data, arr_4k_data)) = try_join!(metadata, history, data)?;

        Ok(CompleteMediaItem {
            title: details.title.clone(),
            media_type: self.media_type,
            request: self.request,
            history,
            arr_data,
            arr_4k_data,
        })
    }

    pub fn is_available(&self) -> bool {
        match &self.media_status {
            MediaStatus::Available | MediaStatus::PartiallyAvailable => true,
            _ => false,
        }
    }

    pub fn has_manager_active(&self) -> bool {
        match &self.media_type {
            MediaType::Movie => arr::movie_manger_active() || arr::movie_4k_manager_active(),
            MediaType::Tv => arr::tv_manager_active() || arr::tv_4k_manager_active(),
        }
    }

    pub fn user_ignored(&self) -> bool {
        let request = match self.request {
            None => return false,
            Some(ref request) => request,
        };

        let ignored_users = match Config::global().ignored_users {
            None => return false,
            Some(ref users) => users,
        };

        if ignored_users.contains(&request.requested_by)
        // .iter()
        // .any(|user| user.eq(&request.requested_by))
        {
            true
        } else {
            false
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

    async fn retrieve_arr_data(&self) -> Result<(Option<ArrData>, Option<ArrData>)> {
        match (self.manager_id, self.manager_4k_id) {
            (Some(id), Some(id_4k)) => {
                let data_standard = ArrData::get_data(self.media_type, id);
                let data_4k = ArrData::get_4k_data(self.media_type, id_4k);

                let (data_standard, data_4k) = try_join!(data_standard, data_4k)?;

                Ok((Some(data_standard), Some(data_4k)))
            }
            (Some(id), _) => Ok((Some(ArrData::get_data(self.media_type, id).await?), None)),
            (None, Some(id_4k)) => Ok((
                Some(ArrData::get_4k_data(self.media_type, id_4k).await?),
                None,
            )),
            (None, None) => Err(eyre!(
                "No *arr id was found for request. Unable to gather file data."
            )),
        }
    }
}

#[derive(Debug)]
pub struct CompleteMediaItem {
    pub title: String,
    pub media_type: MediaType,
    request: Option<MediaRequest>,
    history: WatchHistory,
    arr_data: Option<ArrData>,
    arr_4k_data: Option<ArrData>,
}

impl CompleteMediaItem {
    pub async fn remove_from_server(self) -> Result<()> {
        if let Some(request) = self.request {
            request.remove_request().await?;
        }

        if let Some(arr_data) = self.arr_data {
            arr_data.remove_data().await?;
        }

        if let Some(arr_data) = self.arr_4k_data {
            arr_data.remove_data().await?;
        }

        Ok(())
    }

    pub fn get_disk_size(&self) -> i64 {
        match (self.arr_data.as_ref(), self.arr_4k_data.as_ref()) {
            (Some(arr_data), None) => arr_data.get_disk_size(),
            (None, Some(arr_data)) => arr_data.get_disk_size(),
            (Some(arr_data), Some(arr_data_4k)) => {
                arr_data.get_disk_size() + arr_data_4k.get_disk_size()
            }
            (None, None) => panic!("Tried to get size of none existant object!"),
        }
    }

    fn print_arr_data(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match (self.arr_data.as_ref(), self.arr_4k_data.as_ref()) {
            (Some(arr_data), None) => write!(f, "\n      {}", arr_data)?,
            (None, Some(arr_data_4k)) => write!(f, "\n       {}", arr_data_4k)?,
            (Some(arr_data), Some(_)) => write!(f, "\n      {}", arr_data)?,
            (None, None) => {
                panic!("Tried to write non-existant item")
            }
        }

        Ok(())
    }

    fn status_4k(&self) -> &str {
        match (self.arr_data.as_ref(), self.arr_4k_data.as_ref()) {
            (Some(_), None) => "",
            (None, Some(_)) => "Only 4K ",
            (Some(_), Some(_)) => "4K ",
            (None, None) => {
                panic!("Tried to write non-existant item")
            }
        }
    }
}

impl Display for CompleteMediaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{} {} {}.",
            self.status_4k().yellow(),
            self.media_type.to_string().blue(),
            self.title.green(),
            human_file_size(self.get_disk_size()).red()
        )?;
        if let Some(ref request) = self.request {
            write!(f, " {}", request)?;
        }

        self.print_arr_data(f)?;

        write!(f, "\n      {}", self.history)?;

        write!(f, "\n")
    }
}
