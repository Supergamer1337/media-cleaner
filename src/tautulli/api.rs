use color_eyre::Result;
use serde::de::DeserializeOwned;

use super::responses::{ResponseArr, ResponseObj};
use crate::config::Config;

trait Response<TypeParam> {
    type TypeParam;
}

// This is the same as get_obj, but returns a ResponseArr instead of a ResponseObj.
// I'm not sure how to make this generic, so I'm just copying the function and changing the return type.
pub async fn get_arr<T>(
    command: &str,
    params: Option<Vec<(String, String)>>,
) -> Result<ResponseArr<T>>
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
            .map(|param| param.0 + "=" + &param.1)
            .collect::<Vec<String>>()
            .join("&");
    let url = format!(
        "{}/api/v2?apikey={}&cmd={}",
        config.url, config.api_key, cmd
    );

    let response = client.get(&url).send().await?.json().await?;

    Ok(response)
}

// This is the same as get_arr, but returns a ResponseObj instead of a ResponseArr.
// I'm not sure how to make this generic, so I'm just copying the function and changing the return type.
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
