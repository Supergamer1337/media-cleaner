use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum MediaType {
    Movie,
    Tv,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Movie => write!(f, "Movie"),
            Self::Tv => write!(f, "TV"),
        }
    }
}
