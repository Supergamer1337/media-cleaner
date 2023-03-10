use color_eyre::Result;
use serde::de::DeserializeOwned;

use crate::{config::Config, utils::create_param_string};

use super::responses::Response;

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<Response<T>>
where
    T: DeserializeOwned,
{
    let config = &Config::global().radarr;
    let client = reqwest::Client::new();
    let params = create_param_string(params);

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
    let config = &Config::global().radarr;
    let client = reqwest::Client::new();
    let params = create_param_string(params);

    client
        .delete(format!("{}/api/v3{}?{}", &config.url, path, params))
        .header("X-Api-Key", &config.api_key)
        .send()
        .await?;

    Ok(())
}
