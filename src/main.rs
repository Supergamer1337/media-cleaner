mod arr;
mod config;
mod media_item;
mod overseerr;
mod shared;
mod tautulli;
mod tmdb;
mod utils;

use color_eyre::{eyre::eyre, Result};
use std::{io, process::Command};

use config::Config;
use dialoguer::MultiSelect;
use media_item::get_requests_data;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    if let Err(err) = Config::read_conf() {
        return Err(eyre!("Failed to read the config, with the following error: {:?}.\nPlease make sure all fields are filled.", err));
    }

    let mut requests = get_requests_data().await?;

    clear_screen()?;

    let items_to_show = Config::global().items_shown;
    let chosen: Vec<usize> = MultiSelect::new()
        .with_prompt("Choose what media to delete (SPACE to select, ENTER to confirm selection)")
        .max_length(items_to_show)
        .items(&requests)
        .interact()?;

    if chosen.len() == 0 {
        println!("No items selected. Exiting...");
        return Ok(());
    }

    clear_screen()?;

    println!("Are you sure you want to delete the following items:");
    chosen.iter().for_each(|selection| {
        if let Some(media_item) = requests.get(*selection) {
            let media_type = media_item.media_type;
            println!("- {} - {}", &media_item.title, media_type.to_string());
        } else {
            println!("- Unknown item");
        }
    });

    println!("\ny/n:");
    let user_input = get_user_input()?;

    if !user_input.starts_with("y") {
        println!("Cancelling...");
        return Ok(());
    }

    for selection in chosen.into_iter().rev() {
        let media_item = requests.swap_remove(selection);
        media_item.remove_from_server().await?;
    }

    Ok(())
}

fn clear_screen() -> Result<()> {
    if cfg!(target_os = "windows") {
        Command::new("cmd").arg("/C").arg("cls").status()?;
        Ok(())
    } else {
        Command::new("clear").status()?;
        Ok(())
    }
}

fn get_user_input() -> Result<String> {
    let mut user_input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut user_input)?;

    Ok(user_input.to_lowercase())
}
