use crate::{
    overseerr::MediaRequest, shared::MediaType, tautulli::WatchHistory, tmdb::ItemMetadata,
};

use super::MediaItem;

impl MediaItem {
    /* ------------------ Getters ------------------ */

    pub(super) fn get_request(&self) -> &Option<MediaRequest> {
        match self {
            Self::Tv { request, .. } | Self::Movie { request, .. } => request,
        }
    }

    pub(super) fn get_rating_key(&self) -> &Option<String> {
        match self {
            Self::Tv { rating_key, .. } | Self::Movie { rating_key, .. } => rating_key,
        }
    }

    pub(crate) fn get_title(&self) -> &Option<String> {
        match self {
            Self::Tv { title, .. } | Self::Movie { title, .. } => title,
        }
    }

    pub(crate) fn get_media_type(&self) -> MediaType {
        match self {
            Self::Tv { .. } => MediaType::Tv,
            Self::Movie { .. } => MediaType::Movie,
        }
    }

    pub(super) fn get_history(&self) -> &Option<WatchHistory> {
        match self {
            Self::Tv { history, .. } | Self::Movie { history, .. } => history,
        }
    }

    /* ------------------ Setters ------------------ */

    pub(super) fn set_history(&mut self, watch_history: Option<WatchHistory>) {
        match self {
            Self::Tv { history, .. } | Self::Movie { history, .. } => *history = watch_history,
        }
    }

    pub(super) fn set_details(&mut self, details: Option<ItemMetadata>) {
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

    /* ------------------ Specialized setters ------------------ */
    pub(super) fn remove_request(&mut self) {
        match self {
            Self::Tv { request, .. } | Self::Movie { request, .. } => {
                *request = None;
            }
        }
    }
}
