use color_eyre::Result;
use tokio::try_join;

use super::MediaItem;
use crate::{
    arr::{self, TvData},
    overseerr::MediaStatus,
    tautulli::{self, WatchHistory},
    tmdb::{self, ItemMetadata},
};

impl MediaItem {
    pub(super) fn is_request_available(&self) -> bool {
        if let Some(request) = self.get_request() {
            match request.media_status {
                MediaStatus::Available | MediaStatus::PartiallyAvailable => true,
                _ => false,
            }
        } else {
            false
        }
    }

    pub(super) async fn get_all_info(&mut self) -> Result<()> {
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
}
