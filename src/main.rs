mod config;
mod overseerr;
mod shared;
mod tautulli;

use color_eyre::{owo_colors::OwoColorize, Result};
use config::Config;
use overseerr::{responses::MediaStatus, MediaRequest};
use shared::MediaType;
use tautulli::WatchHistory;

#[derive(Debug)]
struct MediaItem {
    rating_key: Option<String>,
    media_type: MediaType,
    request: Option<MediaRequest>,
    history: Option<WatchHistory>,
}

impl MediaItem {
    fn from_request(request: MediaRequest) -> Result<Self> {
        Ok(Self {
            rating_key: match request.rating_key {
                Some(ref rating_key) => Some(rating_key.clone()),
                None => None,
            },
            media_type: request.media_type,
            request: Some(request),
            history: None,
        })
    }

    async fn get_history(&mut self) -> Result<()> {
        let rating_key = match &self.rating_key {
            Some(ref rating_key) => rating_key,
            None => return Ok(()),
        };

        let history = tautulli::get_item_watches(rating_key, &self.media_type).await?;

        self.history = Some(history);

        Ok(())
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
        let title = match &self.rating_key {
            Some(ref rating_key) => rating_key,
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

    let mut items: Vec<MediaItem> = requests
        .into_iter()
        .filter_map(|request| match MediaItem::from_request(request) {
            Ok(item) => Some(item),
            Err(err) => {
                eprintln!("Error creating media item: {}", err);
                None
            }
        })
        .collect();

    for item in items.iter_mut() {
        if let Err(err) = item.get_history().await {
            println!(
                "Failed to get history for {} with error {}",
                item.rating_key.as_ref().unwrap(),
                err
            );
        } else {
            println!("{}", item);
        }
    }

    Ok(())
}
