use std::fmt::Debug;

use color_eyre::Result;
use itertools::Itertools;
use serde::{de::DeserializeOwned, ser::SerializeStruct, Serialize};

use super::responses::Response;
use crate::config::Config;

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<Response<T>>
where
    T: DeserializeOwned + Debug,
{
    let config = &Config::global().sonarr;
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

pub async fn delete(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<()> {
    let config = &Config::global().sonarr;
    let client = reqwest::Client::new();
    let params = params
        .unwrap_or(vec![])
        .into_iter()
        .map(|param| format!("{}={}", param.0, param.1))
        .join("&");

    client
        .delete(format!("{}/api/v3{}?{}", &config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?;

    Ok(())
}
