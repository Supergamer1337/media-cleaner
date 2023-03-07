use color_eyre::owo_colors::OwoColorize;

use crate::{overseerr::MediaStatus, tautulli::WatchHistory};

use super::MediaItem;

impl MediaItem {
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
