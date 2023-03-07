use color_eyre::{owo_colors::OwoColorize, Report, Result};
use futures::future;
use itertools::Itertools;
use std::cmp::Ordering;
use tokio::try_join;

use crate::{
    arr::{self, MovieData, TvData},
    overseerr::{self, MediaRequest, MediaStatus},
    shared::MediaType,
    tautulli::{self, WatchHistory},
    tmdb::{self, ItemMetadata},
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

// Getters
impl MediaItem {
    fn get_request(&self) -> &Option<MediaRequest> {
        match self {
            Self::Tv { request, .. } | Self::Movie { request, .. } => request,
        }
    }

    fn get_rating_key(&self) -> &Option<String> {
        match self {
            Self::Tv { rating_key, .. } | Self::Movie { rating_key, .. } => rating_key,
        }
    }

    pub fn get_title(&self) -> &Option<String> {
        match self {
            Self::Tv { title, .. } | Self::Movie { title, .. } => title,
        }
    }

    fn get_media_type(&self) -> MediaType {
        match self {
            Self::Tv { .. } => MediaType::Tv,
            Self::Movie { .. } => MediaType::Movie,
        }
    }

    fn get_history(&self) -> &Option<WatchHistory> {
        match self {
            Self::Tv { history, .. } | Self::Movie { history, .. } => history,
        }
    }
}

// Setters
impl MediaItem {
    fn set_history(&mut self, watch_history: Option<WatchHistory>) {
        match self {
            Self::Tv { history, .. } | Self::Movie { history, .. } => *history = watch_history,
        }
    }

    fn set_details(&mut self, details: Option<ItemMetadata>) {
        if let Some(item_details) = details {
            match self {
                Self::Tv { title, details, .. } | Self::Movie { title, details, .. } => {
                    *title = Some(item_details.name.clone());
                    *details = Some(item_details);
                }
            }
        } else {
            match self {
                Self::Tv { title, details, .. } | Self::Movie { title, details, .. } => {
                    *title = None;
                    *details = None;
                }
            }
        }
    }
}

impl MediaItem {
    pub fn from_request(request: MediaRequest) -> Self {
        let cloned_rating_key = match request.rating_key {
            Some(ref rating_key) => Some(rating_key.clone()),
            None => None,
        };
        match request.media_type {
            MediaType::Tv => MediaItem::Tv {
                title: None,
                rating_key: cloned_rating_key,
                request: Some(request),
                history: None,
                details: None,
                tv_data: None,
            },
            MediaType::Movie => MediaItem::Movie {
                title: None,
                rating_key: cloned_rating_key,
                request: Some(request),
                history: None,
                details: None,
                movie_data: None,
            },
        }
    }

    pub fn is_request_available(&self) -> bool {
        if let Some(request) = self.get_request() {
            match request.media_status {
                MediaStatus::Available | MediaStatus::PartiallyAvailable => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub async fn get_all_info(&mut self) -> Result<()> {
        let history = self.retrieve_history();
        let metadata = self.retrieve_metadata();
        let sonarr = self.retrieve_tv_data();
        let res = try_join!(history, metadata, sonarr)?;

        self.set_history(res.0);
        self.set_details(res.1);
        // self.sonarr_data = res.2;

        Ok(())
    }

    async fn retrieve_history(&self) -> Result<Option<WatchHistory>> {
        let rating_key = match self.get_rating_key() {
            Some(ref rating_key) => rating_key,
            None => return Ok(None),
        };

        Ok(Some(
            tautulli::get_item_watches(rating_key, &self.get_media_type()).await?,
        ))
    }

    async fn retrieve_metadata(&self) -> Result<Option<ItemMetadata>> {
        let request = match self.get_request() {
            Some(ref request) => request,
            None => return Ok(None),
        };

        let tmdb_id = match request.tmdb_id {
            Some(ref tmdb_id) => *tmdb_id,
            None => return Ok(None),
        };

        Ok(Some(
            tmdb::get_metadata(tmdb_id, &self.get_media_type()).await?,
        ))
    }

    async fn retrieve_tv_data(&self) -> Result<Option<TvData>> {
        let request = match self.get_request() {
            Some(ref request) => request,
            None => return Ok(None),
        };

        let tvdb_id = match request.tvdb_id {
            Some(tvdb_id) => tvdb_id,
            None => return Ok(None),
        };

        let tv_data = arr::get_tv_data(tvdb_id).await?;

        Ok(Some(tv_data))
    }

    fn write_watch_history(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let None = self.get_history() {
            writeln!(f, "Has no watch history")?;
            return Ok(());
        }

        if let WatchHistory::Movie(history) = self.get_history().as_ref().unwrap() {
            let watches = &history.watches;
            if watches.len() == 0 {
                writeln!(f, "Has no watch history")?;
                return Ok(());
            }
            writeln!(f, "With Watch History:")?;
            watches.iter().for_each(|watch| {
                writeln!(
                    f,
                    "   * Last watch by {} was at {} with {} watched",
                    watch.display_name.purple().to_string(),
                    watch.last_watched.format("%d-%m-%Y").blue().to_string(),
                    (watch.progress.to_string() + "%").yellow()
                )
                .unwrap_or_else(|err| {
                    eprintln!("   * Failed to write watch line with err {}", err)
                });
            });
            return Ok(());
        }

        if let WatchHistory::TvShow(history) = self.get_history().as_ref().unwrap() {
            let watches = &history.watches;
            if watches.len() == 0 {
                writeln!(f, "Has no watch history")?;
                return Ok(());
            }
            writeln!(f, "With Watch History:")?;
            watches.iter().for_each(|watch| {
                writeln!(
                    f,
                    "   * Last watch by {} was at {} on Season {} Episode {} with {} watched",
                    watch.display_name.purple().to_string(),
                    watch.last_watched.format("%d-%m-%Y").blue().to_string(),
                    watch.season,
                    watch.episode,
                    (watch.progress.to_string() + "%").yellow()
                )
                .unwrap_or_else(|err| {
                    eprintln!("   * Failed to write watch line with err {}", err)
                });
            });
            return Ok(());
        }

        Ok(())
    }
}

impl std::fmt::Display for MediaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let title = match self.get_title() {
            Some(ref title) => title,
            None => "Unknown",
        };
        let media_type = self.get_media_type();

        let media_status = match self.get_request() {
            Some(ref request) => request.media_status,
            None => MediaStatus::Unknown,
        };
        let requested_by = match self.get_request() {
            Some(ref request) => &request.requested_by,
            None => "Unknown",
        };

        writeln!(
            f,
            "{} {} with media status {} by {}",
            media_type.to_string().blue(),
            title.bright_yellow(),
            media_status,
            requested_by.bright_purple()
        )?;

        self.write_watch_history(f)?;

        Ok(())
    }
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
        .filter_map(|future| future.ok())
        .sorted_by(|item1, item2| {
            if let (Some(title), Some(title2)) = (item1.get_title(), item2.get_title()) {
                title.cmp(title2)
            } else {
                Ordering::Less
            }
        })
        .collect())
}
