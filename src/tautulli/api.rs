use color_eyre::Result;
use serde::de::DeserializeOwned;

use super::responses::{ResponseArr, ResponseObj};
use crate::config::Config;

trait Response<TypeParam> {
    type TypeParam;
}

// This is the same as getObj, but returns a ResponseArr instead of a ResponseObj.
// I'm not sure how to make this generic, so I'm just copying the function and changing the return type.
pub async fn getArr<T>(
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

// This is the same as getArr, but returns a ResponseObj instead of a ResponseArr.
// I'm not sure how to make this generic, so I'm just copying the function and changing the return type.
pub async fn getObj<T>(
    command: &str,
    params: Option<Vec<(String, String)>>,
) -> Result<ResponseObj<T>>
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

    let response = client.get(&url).send().await?.text().await?;

    println!("{}", response);

    let response: ResponseObj<T> = serde_json::from_str(&response)?;

    Ok(response)
}
