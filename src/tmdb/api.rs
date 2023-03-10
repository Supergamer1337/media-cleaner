use color_eyre::Result;
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

    let movie_details: T = client.get(path).send().await?.json().await?;

    Ok(movie_details)
}
