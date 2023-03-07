use color_eyre::Result;
use itertools::Itertools;
use serde::de::DeserializeOwned;

use crate::config::Config;

use super::responses::Response;

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<Response<T>>
where
    T: DeserializeOwned,
{
    let config = &Config::global().radarr;
    let client = reqwest::Client::new();
    let params = params
        .unwrap_or(vec![])
        .into_iter()
        .map(|param| format!("{}={}", param.0, param.1))
        .join("&");

    let response = client
        .get(format!("{}/api/v3{}?{}", config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?
        .json()
        .await?;

    Ok(response)
}
