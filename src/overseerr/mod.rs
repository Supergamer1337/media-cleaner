mod api;
mod responses;

use std::fmt::Display;

pub use responses::MediaStatus;

use chrono::prelude::*;
use color_eyre::{owo_colors::OwoColorize, Result};

use crate::{
    overseerr::responses::{MediaRequestResponse, RequestResponse},
    shared::MediaType,
};

#[derive(Debug)]
pub struct MediaRequest {
    pub id: u32,
    pub media_id: u32,
    pub tvdb_id: Option<u32>,
    pub tmdb_id: Option<u32>,
    pub rating_key: Option<String>,
    pub rating_key_4k: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub requested_by: String,
    pub request_status: responses::RequestStatus,
    pub media_status: responses::MediaStatus,
    pub media_type: MediaType,
}

impl MediaRequest {
    pub async fn remove_request(self) -> Result<()> {
        let path = format!("/request/{}", self.id.to_string());
        api::delete(&path).await?;
        Ok(())
    }

    pub async fn get_all() -> Result<Vec<Self>> {
        let response_data: RequestResponse<MediaRequestResponse> = api::get("/request").await?;

        let requests: Vec<Result<MediaRequest>> = response_data
            .results
            .into_iter()
            .map(Self::from_response)
            .collect();

        let requests = requests
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<MediaRequest>>();

        Ok(requests)
    }

    fn from_response(response: MediaRequestResponse) -> Result<Self> {
        let created_at = DateTime::parse_from_rfc3339(&response.created_at)?;
        let updated_at = match &response.updated_at {
            Some(updated_at) => DateTime::parse_from_rfc3339(updated_at)?,
            None => created_at,
        };

        let requested_by = match &response.requested_by.display_name {
            Some(display_name) => display_name.clone(),
            None => response.requested_by.email.clone(),
        };

        Ok(MediaRequest {
            id: response.id,
            media_id: response.media.id,
            tvdb_id: response.media.tvdb_id,
            tmdb_id: response.media.tmdb_id,
            rating_key: response.media.rating_key.clone(),
            rating_key_4k: response.media.rating_key_4k.clone(),
            created_at: created_at.with_timezone(&Utc),
            updated_at: updated_at.with_timezone(&Utc),
            request_status: response.status,
            media_status: response.media.status,
            media_type: response.media.media_type,
            requested_by,
        })
    }
}

impl Display for MediaRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Requested by {} at {}, which is {} days ago.",
            self.requested_by.yellow(),
            self.updated_at.format("%d/%m/%Y").blue(),
            Utc::now()
                .signed_duration_since(self.updated_at)
                .num_days()
                .red()
        )
    }
}
