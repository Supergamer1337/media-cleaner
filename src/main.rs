mod config;
mod overseerr;
mod shared;
mod tautulli;

use color_eyre::owo_colors::OwoColorize;
use config::Config;

#[tokio::main]
async fn main() {
    if let Err(err) = Config::read_conf() {
        eprintln!("Error reading config: {}", err);
        std::process::exit(1);
    }

    let requests = overseerr::get_requests().await.unwrap();

    for request in requests.iter() {
        println!(
            "{} {} {:?}",
            request
                .rating_key
                .as_ref()
                .unwrap_or(&"No Rating Key".to_string()),
            request.requested_by.bright_green(),
            request.media_status.bright_blue()
        );

        if let Some(rating_key) = request.rating_key.as_ref() {
            let item_history = tautulli::get_item_watches(rating_key.as_str(), &request.media_type)
                .await
                .unwrap();

            match item_history {
                tautulli::WatchHistory::Movie(item_history) => {
                    println!("{} watches", item_history.watches.len());
                    println!("{:#?}", item_history);
                }
                tautulli::WatchHistory::TvShow(item_history) => {
                    println!("{} watches", item_history.watches.len());
                    println!("{:#?}", item_history);
                }
            }
        }
    }
}
