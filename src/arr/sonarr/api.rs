use std::fmt::Debug;

use color_eyre::{eyre::eyre, Result};
use serde::de::DeserializeOwned;

use super::responses::Response;
use crate::{config::Config, utils::create_param_string};

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<Response<T>>
where
    T: DeserializeOwned + Debug,
{
    let config = &Config::global().sonarr;
    let client = reqwest::Client::new();
    let params = create_param_string(params);

    let response = client
        .get(format!("{}/api/v3{}?{}", config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?;

    if !(response.status().as_u16() >= 200 && response.status().as_u16() < 300) {
        return Err(eyre!(
            "Response did from Sonarr did not have a valid 200-class response. Check your API key."
        ));
    }

    let response = response.json().await?;

    Ok(response)
}

pub async fn delete(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<()> {
    let config = &Config::global().sonarr;
    let client = reqwest::Client::new();
    let params = create_param_string(params);

    client
        .delete(format!("{}/api/v3{}?{}", &config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?;

    Ok(())
}
