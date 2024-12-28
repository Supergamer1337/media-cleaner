mod api;
mod responses;

use crate::overseerr::responses::{MediaRequestResponse, RequestResponse};
use chrono::prelude::*;
use color_eyre::Result;

pub async fn get_all() -> Result<Vec<MediaRequest>> {
    let response_data: RequestResponse<MediaRequestResponse> = api::get("/request", None).await?;

    let requests: Vec<Result<MediaRequest>> = response_data
        .results
        .into_iter()
        .map(MediaRequest::from_response)
        .collect();

    let requests = requests
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<MediaRequest>>();

    Ok(requests)
}

#[derive(Debug)]
pub struct MediaRequest {
    pub id: u32,
    pub media_id: u32,
    pub rating_key: Option<String>,
    pub manager_id: Option<i32>,
    pub manager_4k_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub requested_by: String,
    pub media_status: responses::MediaStatus,
}

impl MediaRequest {
    pub async fn remove_request(self) -> Result<()> {
        let path = format!("/media/{}", self.media_id);
        api::delete(&path).await?;

        Ok(())
    }

    fn from_response(response: MediaRequestResponse) -> Result<Self> {
        let created_at = DateTime::parse_from_rfc3339(&response.created_at)
            .map(|dt| Some(dt.with_timezone(&Utc)))
            .unwrap_or(None);

        let updated_at = if let Some(ref updated_at) = response.updated_at {
            DateTime::parse_from_rfc3339(updated_at)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .unwrap_or(created_at)
        } else {
            created_at
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
            manager_4k_id: response.media.external_service_id_4k,
            created_at,
            updated_at,
            media_status: response.media.status,
            requested_by,
        })
    }
}
