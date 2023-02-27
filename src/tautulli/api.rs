use color_eyre::Result;
use serde::de::DeserializeOwned;

use super::responses::Response;
use crate::config::Config;

pub async fn get<T>(command: &str, params: Option<Vec<&str>>) -> Result<Response<T>>
where
    T: DeserializeOwned,
{
    let config = &Config::global().tautulli;
    let client = reqwest::Client::new();

    let cmd = command.to_string() + "&" + &params.unwrap_or(vec![]).join("&");
    let url = format!(
        "{}/api/v2?apikey={}&cmd={}",
        config.url, config.api_key, cmd
    );

    let response = client.get(&url).send().await?.text().await?;

    println!("{:?}", response);

    let response = serde_json::from_str(&response)?;

    Ok(response)
}
