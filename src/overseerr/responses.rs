use serde::Deserialize;
use serde_repr::Deserialize_repr;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestResponse<T> {
    pub page_info: PageInfo,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    pub page: u32,
    pub pages: u32,
    pub results: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: u32,
    pub email: String,
    pub display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaResponse {
    pub id: u32,
    pub tmdb_id: Option<u32>,
    pub tvdb_id: Option<u32>,
    pub status: MediaStatus,
    pub media_type: MediaType,
}

#[derive(Debug, Deserialize_repr, Clone, Copy)]
#[repr(u8)]
pub enum MediaStatus {
    Unknown = 1,
    Pending,
    Processing,
    PartiallyAvailable,
    Available,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Movie,
    Tv,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaRequestResponse {
    pub id: u32,
    pub media: MediaResponse,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub requested_by: UserResponse,
    pub status: RequestStatus,
}

#[derive(Debug, Deserialize_repr, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[repr(u8)]
pub enum RequestStatus {
    PendingApproval = 1,
    Approved,
    Declined,
}
