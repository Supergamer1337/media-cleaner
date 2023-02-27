mod api;
mod responses;

use chrono::prelude::*;
use color_eyre::Result;

use crate::{
    config::Config,
    overseerr::responses::{MediaRequestResponse, RequestResponse},
};

#[derive(Debug)]
pub struct MediaRequest {
    pub id: u32,
    pub tvdb_id: Option<u32>,
    pub tmdb_id: Option<u32>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub requested_by: String,
    pub request_status: responses::RequestStatus,
    pub media_status: responses::MediaStatus,
    pub media_type: responses::MediaType,
}

pub async fn get_requests(config: &Config) -> Result<Vec<MediaRequest>> {
    let response_data: RequestResponse<MediaRequestResponse> = api::get("/request", config).await?;

    let requests: Vec<Result<MediaRequest>> = response_data
        .results
        .iter()
        .map(response_to_media_request)
        .collect();

    let total_requests = requests.len();

    let requests = requests
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<MediaRequest>>();

    let successful_requests = requests.len();

    if total_requests != successful_requests {
        println!(
            "Failed to parse {} requests out of {}",
            total_requests - successful_requests,
            total_requests
        );
    }

    Ok(requests)
}

fn response_to_media_request(request: &MediaRequestResponse) -> Result<MediaRequest> {
    let created_at = DateTime::parse_from_rfc3339(&request.created_at)?;
    let updated_at = match &request.updated_at {
        Some(updated_at) => DateTime::parse_from_rfc3339(updated_at)?,
        None => created_at,
    };

    let requested_by = match &request.requested_by.display_name {
        Some(display_name) => display_name.clone(),
        None => request.requested_by.email.clone(),
    };

    Ok(MediaRequest {
        id: request.id,
        tvdb_id: request.media.tvdb_id,
        tmdb_id: request.media.tmdb_id,
        created_at: created_at.with_timezone(&Local),
        updated_at: updated_at.with_timezone(&Local),
        request_status: request.status,
        media_status: request.media.status,
        media_type: request.media.media_type,
        requested_by,
    })
}
