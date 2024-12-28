mod api;
mod responses;

use color_eyre::Result;

pub use self::responses::MovieResource;
pub use self::responses::MovieStatus;

pub async fn get_all_items(is_4k: bool) -> Result<Vec<MovieResource>> {
    api::get("/movie", None, is_4k).await
}

pub async fn get_radarr_data(id: i32, is_4k: bool) -> Result<MovieResource> {
    let path = format!("/movie/{}", id.to_string());
    api::get(&path, None, is_4k).await
}

pub async fn delete_radarr_data_and_files(radarr_id: i32) -> Result<()> {
    let path = format!("/movie/{}", radarr_id.to_string());
    let params = vec![("deleteFiles", "true"), ("addImportExclusion", "false")];
    api::delete(path.as_str(), Some(params)).await
}
