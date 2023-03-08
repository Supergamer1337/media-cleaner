mod arr;
mod config;
mod media_item;
mod overseerr;
mod shared;
mod tautulli;
mod tmdb;

use std::process::Command;

use color_eyre::Result;

use config::Config;
use dialoguer::MultiSelect;
use media_item::get_requests_data;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    if let Err(err) = Config::read_conf() {
        eprintln!("Error reading config: {}", err);
        std::process::exit(1);
    }

    let mut requests = get_requests_data().await?;

    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status()?;
    } else {
        Command::new("clear").status()?;
    };

    let chosen: Vec<usize> = MultiSelect::new()
        .with_prompt("Choose what media to delete.")
        .max_length(5)
        .items(&requests)
        .interact()?;

    for selection in chosen.into_iter().rev() {
        let media_item = requests.swap_remove(selection);
        media_item.delete_item().await?;
    }

    Ok(())
}
