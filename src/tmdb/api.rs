use color_eyre::Result;
use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::config::Config;

pub async fn get<T>(path: &str, query_params: Option<Vec<(&str, &str)>>) -> Result<T>
where
    T: DeserializeOwned,
{
    let config = &Config::global().tmdb;
    let client = Client::new();
    let params = query_params
        .unwrap_or(vec![])
        .into_iter()
        .map(|param| format!("{}={}", param.0, param.1))
        .collect::<Vec<String>>()
        .join("&");
    let path = format!(
        "https://api.themoviedb.org/3/{}?api_key={}&{}",
        path, config.v3_key, params
    );

    let movie_details: T = client.get(path).send().await?.json().await?;

    Ok(movie_details)
}
