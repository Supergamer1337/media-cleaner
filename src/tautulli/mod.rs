mod api;
mod responses;

use chrono::prelude::*;
use color_eyre::Result;

use crate::{shared::MediaType, tautulli::responses::ResponseObj};

use self::responses::{History, HistoryItem, HistoryMovie};

pub struct WatchHistory {
    pub rating_key: String,
    pub media_type: String,
    pub last_watched_history: Vec<UserLastWatch>,
}

pub struct UserLastWatch {
    pub display_name: String,
    pub last_watched: DateTime<Local>,
    pub progress: u32,
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

pub async fn get_item_watches(rating_key: &str, media_type: &MediaType) -> Result<()> {
    let history = get_item_history(rating_key, media_type).await?;

    let latest_user_history = history.data.iter().fold(vec![], latest_watch);
    println!("{:?}", latest_user_history);

    Ok(())
}

async fn get_item_history(rating_key: &str, media_type: &MediaType) -> Result<History> {
    if let MediaType::Movie = media_type {
        let params = vec![("rating_key".to_string(), rating_key.to_string())];
        let history: ResponseObj<HistoryMovie> = api::get_obj("get_history", Some(params)).await?;
        Ok(history_movie_to_history(history.response.data))
    } else {
        let params = vec![("grandparent_rating_key".to_string(), rating_key.to_string())];
        let history: ResponseObj<History> = api::get_obj("get_history", Some(params)).await?;
        Ok(history.response.data)
    }
}

fn history_movie_to_history(history: HistoryMovie) -> History {
    History {
        draw: history.draw,
        records_total: history.records_total,
        records_filtered: history.records_filtered,
        data: history
            .data
            .into_iter()
            .map(|item| HistoryItem {
                user: item.user,
                date: item.date,
                duration: item.duration,
                percent_complete: item.percent_complete,
                media_index: None,
                parent_media_index: None,
            })
            .collect(),
    }
}

fn latest_watch<'a>(
    mut latest_watches_so_far: Vec<&'a HistoryItem>,
    current_watch: &'a HistoryItem,
) -> Vec<&'a HistoryItem> {
    let prev_history_item = latest_watches_so_far
        .iter()
        .find(|last_watch| last_watch.user == current_watch.user);

    match prev_history_item {
        None => {
            latest_watches_so_far.push(current_watch);
            latest_watches_so_far
        }
        Some(user_last_watch) if user_last_watch.date > current_watch.date => latest_watches_so_far,
        Some(_) => {
            let index_to_remove = latest_watches_so_far
                .iter()
                .position(|last_watch| last_watch.user == current_watch.user)
                .expect("Failed to find index of user in history");
            latest_watches_so_far.swap_remove(index_to_remove);
            latest_watches_so_far.push(current_watch);
            latest_watches_so_far
        }
    }
}
