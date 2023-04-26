mod api;
mod responses;

use chrono::prelude::*;
use color_eyre::{eyre::eyre, owo_colors::OwoColorize, Result};
use std::fmt::Display;

use self::responses::MediaResponse;
use crate::{
    overseerr::responses::{MediaRequestResponse, RequestResponse},
    shared::MediaType,
};
pub use responses::MediaStatus;

#[derive(Debug)]
pub struct MediaRequest {
    pub id: u32,
    pub media_id: u32,
    pub rating_key: Option<String>,
    pub manager_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub requested_by: String,
    pub media_status: responses::MediaStatus,
    pub media_type: MediaType,
}

impl MediaRequest {
    pub async fn remove_request(self) -> Result<()> {
        let path = format!("/media/{}", self.media_id);
        api::delete(&path).await?;

        Ok(())
    }

    pub async fn get_all() -> Result<Vec<Self>> {
        let response_data: RequestResponse<MediaRequestResponse> =
            api::get("/request", None).await?;

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
            rating_key: response.media.rating_key,
            manager_id: response.media.external_service_id,
            created_at: created_at.with_timezone(&Utc),
            updated_at: updated_at.with_timezone(&Utc),
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

#[derive(Debug)]
pub struct ServerItem {
    pub id: u32,
    pub rating_key: String,
    pub manager_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub media_status: responses::MediaStatus,
    pub media_type: MediaType,
}

impl ServerItem {
    pub async fn get_all() -> Result<Vec<Self>> {
        let response_data: RequestResponse<MediaResponse> =
            api::get("/media", Some(vec![("filter", "available")])).await?;

        let requests: Vec<Result<Self>> = response_data
            .results
            .into_iter()
            .map(Self::from_response)
            .collect();

        let requests = requests
            .into_iter()
            .filter_map(Result::ok)
            .collect::<Vec<Self>>();

        Ok(requests)
    }

    fn from_response(response: MediaResponse) -> Result<Self> {
        let created_at = DateTime::parse_from_rfc3339(&response.created_at)?;
        let updated_at = match &response.updated_at {
            Some(updated_at) => DateTime::parse_from_rfc3339(updated_at)?,
            None => created_at,
        };

        Ok(Self {
            id: response.id,
            rating_key: match response.rating_key {
                Some(rating_key) => rating_key,
                None => {
                    return Err(eyre!(
                        "No rating key found for item {} of type {}.",
                        response.id,
                        response.media_type
                    ))
                }
            },
            manager_id: response.external_service_id,
            created_at: created_at.with_timezone(&Utc),
            updated_at: updated_at.with_timezone(&Utc),
            media_status: response.status,
            media_type: response.media_type,
        })
    }
}
