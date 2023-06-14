use color_eyre::{eyre::eyre, Result};
use serde::de::DeserializeOwned;
use serde_json::from_str;

use crate::{
    config::{Config, Radarr},
    utils::{create_api_error_message, create_param_string},
};

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>, is_4k: bool) -> Result<T>
where
    T: DeserializeOwned,
{
    let config: &Radarr;
    if is_4k {
        config = match &Config::global().radarr_4k {
            Some(ref radarr) => radarr,
            None => {
                return Err(eyre!(
                    "Tried to access radarr config, even though it is not defined."
                ))
            }
        }
    } else {
        config = match &Config::global().radarr {
            Some(ref radarr) => radarr,
            None => {
                return Err(eyre!(
                    "Tried to access radarr config, even though it is not defined."
                ))
            }
        };
    }

    let client = reqwest::Client::new();
    let params = create_param_string(params);

    let response = client
        .get(format!("{}/api/v3{}?{}", config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?;

    if !(response.status().as_u16() >= 200 && response.status().as_u16() < 300) {
        let code = response.status().as_u16();
        return Err(eyre!(create_api_error_message(code, path, "Radarr")));
    }

    let response = response.json().await?;

    Ok(response)
}

pub async fn delete(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<()> {
    let config = match &Config::global().radarr {
        Some(ref radarr) => radarr,
        None => {
            return Err(eyre!(
                "Tried to access radarr config, even though it is not defined."
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
