use color_eyre::Result;
use serde::de::DeserializeOwned;

use crate::{
    config::Config,
    utils::{create_api_error_message, create_param_string},
};
use color_eyre::eyre::eyre;

pub async fn get<T>(path: &str, params: Option<Vec<(&str, &str)>>) -> Result<T>
where
    T: DeserializeOwned,
{
    let config = &Config::global().plex;
    let client = reqwest::Client::new();
    let params = create_param_string(params);

    let response = client
        .get(format!(
            "{}{}?X-Plex-Token={}&{}",
            config.url, path, config.token, params
        ))
        .send()
        .await?;

    if !(response.status().as_u16() >= 200 && response.status().as_u16() < 300) {
        let code = response.status().as_u16();
        return Err(eyre!(create_api_error_message(code, path, "Plex")));
    }

    let response_text = response.text().await?;
    let parsed_response: T = serde_xml_rs::from_str(&response_text)?;

    Ok(parsed_response)
}
