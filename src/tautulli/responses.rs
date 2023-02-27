use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Response<T> {
    response: ResponseInternal<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseInternal<T> {
    pub message: Option<String>,
    pub result: ResultType,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ResultType {
    Success,
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct User {
    username: String,
    user_id: u32,
}
