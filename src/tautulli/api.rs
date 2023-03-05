use color_eyre::Result;
use serde::de::DeserializeOwned;

use super::responses::ResponseObj;
use crate::config::Config;

trait Response<TypeParam> {
    type TypeParam;
}

pub async fn get_obj<T>(command: &str, params: Option<Vec<(&str, &str)>>) -> Result<ResponseObj<T>>
where
    T: DeserializeOwned,
{
    let config = &Config::global().tautulli;
    let client = reqwest::Client::new();

    let cmd = command.to_string()
        + "&"
        + &params
            .unwrap_or(vec![])
            .into_iter()
            .map(|param| format!("{}={}", param.0, param.1))
            .collect::<Vec<String>>()
            .join("&");

    let url = format!(
        "{}/api/v2?apikey={}&cmd={}",
        config.url, config.api_key, cmd
    );

    let response = client.get(&url).send().await?.json().await?;

    Ok(response)
}
