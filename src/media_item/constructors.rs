use crate::{overseerr::MediaRequest, shared::MediaType};

use super::MediaItem;

impl MediaItem {
    pub(super) fn from_request(request: MediaRequest) -> Self {
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
}
