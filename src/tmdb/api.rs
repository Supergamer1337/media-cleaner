use color_eyre::{eyre::eyre, Result};
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::{config::Config, utils::create_param_string};

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<T>
where
    T: DeserializeOwned,
{
    let config = &Config::global().tmdb;
    let client = Client::new();
    let params = create_param_string(params);
    let path = format!(
        "https://api.themoviedb.org/3/{}?api_key={}&{}",
        path, config.v3_key, params
    );

    let response = client.get(path).send().await?;

    if !(response.status().as_u16() >= 200 && response.status().as_u16() < 300) {
        return Err(eyre!(
            "Response did from TMDB did not have a valid 200-class response. Check your API key."
        ));
    }

    let response = response.json().await?;

    Ok(response)
}
