mod config;
mod overseerr;
mod shared;
mod tautulli;
mod tmdb;

use color_eyre::{owo_colors::OwoColorize, Report, Result};
use futures::future;
use itertools::Itertools;

use config::Config;
use overseerr::{responses::MediaStatus, MediaRequest};
use shared::MediaType;
use tautulli::WatchHistory;
use tmdb::ItemMetadata;
use tokio::try_join;

#[derive(Debug)]
struct MediaItem {
    title: Option<String>,
    rating_key: Option<String>,
    media_type: MediaType,
    request: Option<MediaRequest>,
    history: Option<WatchHistory>,
    details: Option<ItemMetadata>,
}

impl MediaItem {
    fn from_request(request: MediaRequest) -> Result<Self> {
        Ok(Self {
            title: None,
            rating_key: match request.rating_key {
                Some(ref rating_key) => Some(rating_key.clone()),
                None => None,
            },
            media_type: request.media_type,
            request: Some(request),
            history: None,
            details: None,
        })
    }

    async fn get_all_info(&mut self) -> Result<()> {
        let history = self.retrieve_history();
        let metadata = self.retrieve_metadata();
        let res = try_join!(history, metadata)?;

        self.history = res.0;
        if let Some(details) = res.1 {
            self.title = Some(details.name.clone());
            self.details = Some(details);
        }

        Ok(())
    }

    async fn get_history(&mut self) -> Result<()> {
        let history = self.retrieve_history().await?;
        self.history = history;

        Ok(())
    }

    async fn retrieve_history(&self) -> Result<Option<WatchHistory>> {
        let rating_key = match &self.rating_key {
            Some(ref rating_key) => rating_key,
            None => return Ok(None),
        };

        Ok(Some(
            tautulli::get_item_watches(rating_key, &self.media_type).await?,
        ))
    }

    async fn get_item_metadata(&mut self) -> Result<()> {
        let details = self.retrieve_metadata().await?;
        match details {
            Some(details) => {
                self.title = Some(details.name.clone());
                self.details = Some(details);
                Ok(())
            }
            None => Ok(()),
        }
    }

    async fn retrieve_metadata(&self) -> Result<Option<ItemMetadata>> {
        let request = match self.request {
            Some(ref request) => request,
            None => return Ok(None),
        };

        let tmdb_id = match request.tmdb_id {
            Some(ref tmdb_id) => *tmdb_id,
            None => return Ok(None),
        };

        Ok(Some(tmdb::get_metadata(tmdb_id, &self.media_type).await?))
    }

    fn write_watch_history(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let None = self.history {
            writeln!(f, "Has no watch history")?;
            return Ok(());
        }

        if let WatchHistory::Movie(history) = self.history.as_ref().unwrap() {
            let watches = &history.watches;
            if watches.len() == 0 {
                writeln!(f, "Has no watch history")?;
                return Ok(());
            }
            writeln!(f, "With Watch History:")?;
            watches.iter().for_each(|watch| {
                writeln!(
                    f,
                    "   * Last watch by {} was at {}",
                    watch.display_name.purple().to_string(),
                    watch.last_watched.format("%d-%m-%Y").blue().to_string(),
                )
                .unwrap_or_else(|err| {
                    eprintln!("   * Failed to write watch line with err {}", err)
                });
            });
            return Ok(());
        }

        if let WatchHistory::TvShow(history) = self.history.as_ref().unwrap() {
            let watches = &history.watches;
            if watches.len() == 0 {
                writeln!(f, "Has no watch history")?;
            }
            writeln!(f, "With Watch History:")?;
            watches.iter().for_each(|watch| {
                writeln!(
                    f,
                    "   * Last watch by {} was at {}, Season {} Episode {}",
                    watch.display_name.purple().to_string(),
                    watch.last_watched.format("%d-%m-%Y").blue().to_string(),
                    watch.season,
                    watch.episode
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
        let title = match self.title {
            Some(ref title) => title,
            None => "Unknown",
        };
        let media_type = &self.media_type;

        let media_status = match self.request {
            Some(ref request) => request.media_status,
            None => MediaStatus::Unknown,
        };
        let requested_by = match self.request {
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

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    if let Err(err) = Config::read_conf() {
        eprintln!("Error reading config: {}", err);
        std::process::exit(1);
    }

    let requests = overseerr::get_requests().await.unwrap();

    let items: Vec<MediaItem> = requests
        .into_iter()
        .filter_map(|request| match MediaItem::from_request(request) {
            Ok(item) => Some(item),
            Err(err) => {
                eprintln!("Error creating media item: {}", err);
                None
            }
        })
        .collect();

    let futures = items.into_iter().map(|mut item| {
        tokio::spawn(async move {
            item.get_all_info().await?;

            Ok::<MediaItem, Report>(item)
        })
    });

    let items: Vec<MediaItem> = future::try_join_all(futures)
        .await?
        .into_iter()
        .filter_map(|future| future.ok())
        .collect();

    items
        .iter()
        .unique_by(|item| item.title.as_ref().unwrap())
        .for_each(|item| {
            println!("{}", item);
        });

    Ok(())
}
