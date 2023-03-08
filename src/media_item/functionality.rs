use color_eyre::{eyre::eyre, Result};
use tokio::try_join;

use super::MediaItem;
use crate::{
    arr::{self, MovieData, TvData},
    config::Sonarr,
    overseerr::{self, MediaStatus},
    tautulli::{self, WatchHistory},
    tmdb::{self, ItemMetadata},
};

enum ArrData {
    Movie(MovieData),
    Tv(TvData),
}

impl MediaItem {
    pub(crate) async fn delete_item(mut self) -> Result<()> {
        self.delete_request().await?;
        self.delete_arr_data().await?;

        Ok(())
    }

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
        let data = self.retrieve_arr_data();
        let res = try_join!(history, metadata, data)?;

        self.set_history(res.0);
        self.set_details(res.1);

        if let Some(arr_data) = res.2 {
            match arr_data {
                ArrData::Movie(new_movie_data) => {
                    if let Self::Movie { movie_data, .. } = self {
                        *movie_data = Some(new_movie_data);
                    } else {
                        return Err(eyre!("Tried setting movie data to a tv show."));
                    }
                }
                ArrData::Tv(new_tv_data) => {
                    if let Self::Tv { tv_data, .. } = self {
                        *tv_data = Some(new_tv_data);
                    } else {
                        return Err(eyre!("Tried setting tv data to a movie."));
                    }
                }
            }
        }

        Ok(())
    }

    async fn delete_request(&mut self) -> Result<()> {
        let request = match self.get_request() {
            Some(request) => request,
            None => return Ok(()),
        };

        overseerr::remove_request(request.id).await?;

        self.remove_request();

        Ok(())
    }

    async fn delete_arr_data(&mut self) -> Result<()> {
        match self {
            Self::Tv { tv_data, .. } => {
                if let Some(tv_data) = tv_data {
                    arr::remove_tv_data(tv_data.id).await?;
                    Ok(())
                } else {
                    Ok(())
                }
            }
            Self::Movie { movie_data, .. } => {
                if let Some(movie_data) = movie_data {
                    arr::remove_movie_data(movie_data.id).await?;
                    Ok(())
                } else {
                    Ok(())
                }
            }
        }
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

    async fn retrieve_arr_data(&self) -> Result<Option<ArrData>> {
        let request = match self.get_request() {
            Some(ref request) => request,
            None => return Ok(None),
        };

        match self {
            Self::Tv { .. } => {
                let tvdb_id = match request.tvdb_id {
                    Some(tvdb_id) => tvdb_id,
                    None => return Ok(None),
                };

                let tv_data = arr::get_tv_data(tvdb_id).await?;

                Ok(Some(ArrData::Tv(tv_data)))
            }
            Self::Movie { .. } => {
                let tmdb_id = match request.tmdb_id {
                    Some(tmdb_id) => tmdb_id,
                    None => return Ok(None),
                };

                let movie_data = arr::get_movie_data(tmdb_id).await?;

                Ok(Some(ArrData::Movie(movie_data)))
            }
        }
    }
}
