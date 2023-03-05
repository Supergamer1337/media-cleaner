use color_eyre::Result;
use serde::de::DeserializeOwned;

use super::responses::RequestResponse;
use crate::config::Config;

pub async fn get<T>(path: &str) -> Result<RequestResponse<T>>
where
    T: DeserializeOwned,
{
    let client = reqwest::Client::new();
    let config = Config::global();

    let mut response_data: RequestResponse<T> = client
        .get(format!("{}/api/v1{}?take=100", &config.overseerr.url, path))
        .header("X-API-Key", &config.overseerr.api_key)
        .send()
        .await?
        .json()
        .await?;

    let page_size = response_data.page_info.page_size;
    for page in 1..response_data.page_info.pages {
        let mut page_data: RequestResponse<T> = client
            .get(format!(
                "{}/api/v1{}?take={}&skip={}",
                &config.overseerr.url,
                path,
                page_size,
                page_size * page
            ))
            .header("X-API-Key", &config.overseerr.api_key)
            .send()
            .await?
            .json()
            .await?;

        response_data.results.append(&mut page_data.results);
    }

    Ok(response_data)
}
