use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseArr<T> {
    pub response: ResponseInternalArr<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseInternalArr<T> {
    pub message: Option<String>,
    pub result: ResultType,
    pub data: Vec<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseObj<T> {
    pub response: ResponseInternalObj<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct ResponseInternalObj<T> {
    pub message: Option<String>,
    pub result: ResultType,
    pub data: T,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ResultType {
    Success,
    Error,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryMovie {
    pub draw: u32,
    pub records_total: u32,
    pub records_filtered: u32,
    pub data: Vec<HistoryMovieItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HistoryMovieItem {
    pub date: i64,
    pub duration: u64,
    pub percent_complete: u8,
    pub user: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub draw: u32,
    pub records_total: u32,
    pub records_filtered: u32,
    pub data: Vec<HistoryItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct HistoryItem {
    pub user: String,
    pub date: i64,
    pub duration: u64,
    pub percent_complete: u8,
    pub media_index: Option<u32>,
    pub parent_media_index: Option<u32>,
}
