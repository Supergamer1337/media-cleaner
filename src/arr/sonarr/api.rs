use std::fmt::Debug;

use color_eyre::{eyre::eyre, Result};
use serde::de::DeserializeOwned;

use crate::{
    config::Config,
    utils::{create_api_error_message, create_param_string},
};

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<T>
where
    T: DeserializeOwned + Debug,
{
    let config = match &Config::global().sonarr {
        Some(sonarr) => sonarr,
        None => {
            return Err(eyre!(
                "Tried to access Sonarr config, even though it is not defined."
            ))
        }
    };
    let client = reqwest::Client::new();
    let params = create_param_string(params);

    let response = client
        .get(format!("{}/api/v3{}?{}", config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?;

    if !(response.status().as_u16() >= 200 && response.status().as_u16() < 300) {
        let code = response.status().as_u16();
        return Err(eyre!(create_api_error_message(code, path, "Sonarr")));
    }

    let response = response.json().await?;

    Ok(response)
}

pub async fn delete(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<()> {
    let config = match &Config::global().sonarr {
        Some(sonarr) => sonarr,
        None => {
            return Err(eyre!(
                "Tried to access Sonarr config, even though it is not defined."
            ))
        }
    };
    let client = reqwest::Client::new();
    let params = create_param_string(params);

    client
        .delete(format!("{}/api/v3{}?{}", &config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?;

    Ok(())
}
