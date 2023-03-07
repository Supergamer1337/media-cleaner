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

    let requests = get_requests_data().await?;

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

    chosen.into_iter().for_each(|selection| {
        requests.get(selection).map(|request| {
            println!("Deleting {}", request.get_title().as_ref().unwrap());
        });
    });

    Ok(())
}
