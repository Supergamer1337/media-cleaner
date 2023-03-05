mod config;
mod media_item;
mod overseerr;
mod shared;
mod tautulli;
mod tmdb;

use std::cmp::Ordering;

use color_eyre::{Report, Result};
use futures::future;
use itertools::Itertools;

use config::Config;
use media_item::MediaItem;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    if let Err(err) = Config::read_conf() {
        eprintln!("Error reading config: {}", err);
        std::process::exit(1);
    }

    let requests = overseerr::get_requests().await.unwrap();

    let items = requests
        .into_iter()
        .filter_map(|request| match MediaItem::from_request(request) {
            Ok(item) => Some(item),
            Err(err) => {
                eprintln!("Error creating media item: {}", err);
                None
            }
        })
        .filter(|item| item.is_request_available())
        .sorted_by(|item1, item2| {
            if let (Some(rating_key), Some(rating_key2)) =
                (item1.get_rating_key(), item2.get_rating_key())
            {
                rating_key.cmp(rating_key2)
            } else {
                Ordering::Less
            }
        })
        .dedup_by(|item1, item2| {
            if let (Some(rating_key), Some(rating_key2)) =
                (item1.get_rating_key(), item2.get_rating_key())
            {
                rating_key.eq(rating_key2)
            } else {
                false
            }
        })
        .collect_vec();

    let futures = items.into_iter().map(|mut item| {
        tokio::spawn(async move {
            item.get_all_info().await?;

            Ok::<MediaItem, Report>(item)
        })
    });

    let items = future::try_join_all(futures)
        .await?
        .into_iter()
        .filter_map(|future| future.ok())
        .collect_vec();

    items
        .iter()
        .sorted_by(|item1, item2| {
            if let (Some(title), Some(title2)) = (item1.get_title(), item2.get_title()) {
                title.cmp(title2)
            } else {
                Ordering::Less
            }
        })
        .for_each(|item| {
            println!("{}", item);
        });

    Ok(())
}
