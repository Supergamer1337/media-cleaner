use color_eyre::{eyre::eyre, Result};
use serde::de::DeserializeOwned;

use super::responses::ResponseObj;
use crate::{
    config::Config,
    utils::{create_api_error_message, create_param_string},
};

trait Response<TypeParam> {
    type TypeParam;
}

pub async fn get_obj<T>(command: &str, params: Option<Vec<(&str, &str)>>) -> Result<ResponseObj<T>>
where
    T: DeserializeOwned,
{
    let config = &Config::global().tautulli;
    let client = reqwest::Client::new();

    let cmd = command.to_string() + "&" + &create_param_string(params);

    let url = format!(
        "{}/api/v2?apikey={}&cmd={}",
        config.url, config.api_key, cmd
    );

    let response = client.get(&url).send().await?;

    if !(response.status().as_u16() >= 200 && response.status().as_u16() < 300) {
        let code = response.status().as_u16();
        return Err(eyre!(create_api_error_message(code, &url, "Tautulli")));
    }

    let response = response.json().await?;

    Ok(response)
}
