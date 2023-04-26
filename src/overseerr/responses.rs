use color_eyre::owo_colors::OwoColorize;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::fmt::Display;

use crate::shared::MediaType;

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
    pub page_size: u32,
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
    pub external_service_id: Option<i32>,
    pub rating_key: Option<String>,
    pub status: MediaStatus,
    pub media_type: MediaType,
    pub created_at: String,
    pub updated_at: Option<String>,
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

impl Display for MediaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "{}", "Unknown".red().to_string()),
            Self::Pending => write!(f, "{}", "Pending".yellow().to_string()),
            Self::Processing => write!(f, "{}", "Processing".yellow().to_string()),
            Self::PartiallyAvailable => write!(f, "{}", "Partially Available".blue().to_string()),
            Self::Available => write!(f, "{}", "Available".green().to_string()),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaRequestResponse {
    pub id: u32,
    pub media: MediaResponse,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub requested_by: UserResponse,
}
