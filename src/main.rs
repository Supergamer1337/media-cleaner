mod arguments;
mod arr;
mod config;
mod media_item;
mod overseerr;
mod plex;
mod shared;
mod tautulli;
mod utils;

use color_eyre::{eyre::eyre, Report, Result};
use futures::future;
use itertools::Itertools;
use overseerr::MediaRequest;
use shared::{Order, SortingOption, SortingValue};
use std::{io, process::Command};

use arguments::Arguments;
use config::Config;
use dialoguer::MultiSelect;
use media_item::{CompleteMediaItem, MediaItem};

use crate::{overseerr::ServerItem, utils::human_file_size};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    read_and_validate_config()?;

    Arguments::read_args()?;

    let mut deletion_items = get_deletion_items().await?;

    show_requests_result(&deletion_items)?;

    clear_screen()?;

    let chosen = choose_items_to_delete(&mut deletion_items)?;

    delete_chosen_items(&mut deletion_items, &chosen).await?;

    Ok(())
}

fn read_and_validate_config() -> Result<()> {
    if let Err(err) = Config::read_conf() {
        return Err(eyre!("Failed to read the config, with the following error: {}.\nPlease make sure all fields are filled.", err));
    }

    let config = Config::global();
    if let (None, None) = (&config.radarr, &config.sonarr) {
        return Err(eyre!("You have not configured Sonarr or Radarr. Application can't continue without at least one of these."));
    }

    Ok(())
}

async fn get_deletion_items() -> Result<Vec<CompleteMediaItem>> {
    println!("Gathering all required data from your services.\nDepending on the amount of data and your connection speed, this could take a while...");

    let all_items = Arguments::get_args().all_media;

    let mut media_items = MediaRequest::get_all()
        .await?
        .into_iter()
        .map(MediaItem::from_request)
        .collect_vec();

    // This is done by merging the two lists, because Overseerr does not send who requested the media along
    // when getting all of the media on the server. Neither does Overseerr have an endpoint for getting all
    // requests associated with an item.
    //
    // If that was allowed, this could be made much nicer and more performance friendly.
    if all_items {
        let mut not_requested_media_items = ServerItem::get_all()
            .await?
            .into_iter()
            .map(MediaItem::from_server_item)
            .collect_vec();

        media_items.append(&mut not_requested_media_items);

        media_items.sort_by(|item1, item2| {
            (&item1.rating_key, item1.request.is_some())
                .cmp(&(&item2.rating_key, item2.request.is_some()))
        });
        media_items.dedup_by(|item1, item2| item1.rating_key == item2.rating_key);
    }

    let futures = media_items
        .into_iter()
        .filter(|i| i.is_available() && i.has_manager_active())
        .map(|item| {
            tokio::spawn(async move {
                let item = item.into_complete_media().await?;

                Ok::<CompleteMediaItem, Report>(item)
            })
        });

    let mut errors: Vec<Report> = Vec::new();

    let complete_items = future::try_join_all(futures)
        .await?
        .into_iter()
        .filter_map(|f| match f {
            Ok(item) => Some(item),
            Err(err) => {
                errors.push(err);
                None
            }
        })
        .unique_by(|item| item.title.clone())
        .sorted_by(|item1, item2| item1.title.cmp(&item2.title))
        .collect();

    show_potential_request_errors(errors)?;

    Ok(complete_items)
}

fn show_potential_request_errors(errs: Vec<Report>) -> Result<()> {
    if errs.len() == 0 {
        return Ok(());
    }

    println!("You got {} errors while gathering data. Press y to show them, or any other input to continue with the errored items ignored.", errs.len());
    let input = get_user_input()?;
    if !input.starts_with("y") {
        return Ok(());
    }

    errs.iter().enumerate().for_each(|(i, err)| {
        println!("Error {} was {}", i, err);
        print_line();
    });

    println!("Do you want to see the full stack traces? Press y. Otherwise continuing to deletion screen with errored items ignored.");
    let inp = get_user_input()?;
    if !inp.starts_with("y") {
        return Ok(());
    }

    errs.iter().enumerate().for_each(|(i, err)| {
        println!("Error {} was {:?}", i + 1, err);
        print_line();
    });

    wait(Some(
        "Press enter to continue to deletion screen with errored items ignored.",
    ))?;

    Ok(())
}

fn show_requests_result(requests: &Vec<CompleteMediaItem>) -> Result<()> {
    if requests.len() == 0 {
        println!("You do not seem to have any valid requests, with data available.");
        println!("Are you sure all your requests are available and downloaded? Or some data was unable to be acquired from other services.");
        println!("Either try again later, or look over your requests.");

        println!();
        wait(None)?;
        std::process::exit(0);
    }

    Ok(())
}

fn choose_items_to_delete(requests: &mut Vec<CompleteMediaItem>) -> Result<Vec<usize>> {
    choose_sorting(requests)?;

    clear_screen()?;

    let items_to_show = Config::global().items_shown;
    let chosen: Vec<usize> = MultiSelect::new()
        .with_prompt("Choose what media to delete (SPACE to select, ENTER to confirm selection)")
        .max_length(items_to_show)
        .items(&requests)
        .interact()?;

    if chosen.len() == 0 {
        println!("No items selected. Exiting...");
        std::process::exit(0);
    }

    clear_screen()?;

    verify_chosen(requests, &chosen)?;

    Ok(chosen)
}

fn choose_sorting(requests: &mut Vec<CompleteMediaItem>) -> Result<()> {
    clear_screen()?;

    let args = Arguments::get_args();

    let sort = match args.sorting {
        Some(ref sort) => sort.clone(),
        None => choose_sorting_dialogue()?,
    };

    match sort.sorting_value {
        SortingValue::Name => (),
        SortingValue::Size => requests.sort_by_key(|req| req.get_disk_size()),
        SortingValue::Type => requests.sort_by_key(|req| req.media_type),
    };

    match sort.sorting_direction {
        Order::Asc => (),
        Order::Desc => requests.reverse(),
    };

    Ok(())
}

fn choose_sorting_dialogue() -> Result<SortingOption> {
    loop {
        println!("Choose sorting method:");
        println!("Name - Ascending: n (or just enter, it's the default)");
        println!("Name - Descending: nd");
        println!("Size - Descending: s");
        println!("Size - Ascending: sa");
        println!("Type - Descending: t");

        let input = get_user_input()?;

        if let Ok(sort) = SortingOption::from_str(&input) {
            return Ok(sort);
        }
        if input.eq("") {
            return Ok(SortingOption::default());
        }
    }
}

fn verify_chosen(requests: &Vec<CompleteMediaItem>, chosen: &Vec<usize>) -> Result<()> {
    let total_size: String = human_file_size(
        chosen
            .iter()
            .filter_map(|selection| {
                if let Some(media_item) = requests.get(*selection) {
                    Some(media_item.get_disk_size())
                } else {
                    None
                }
            })
            .sum(),
    );

    println!(
        "Are you sure you want to delete the following items ({}):",
        total_size
    );
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
        std::process::exit(0);
    }

    Ok(())
}

async fn delete_chosen_items(
    requests: &mut Vec<CompleteMediaItem>,
    chosen: &Vec<usize>,
) -> Result<()> {
    let mut errs: Vec<(String, Report)> = Vec::new();

    for selection in chosen.into_iter().rev() {
        let media_item = requests.swap_remove(*selection);
        let title = media_item.title.clone();
        if let Err(err) = media_item.remove_from_server().await {
            errs.push((title, err));
        }
    }

    if errs.len() > 0 {
        println!("Had some errors deleting items:\n");
        errs.iter().for_each(|err| {
            println!(
                "Got the following error while deleting {}: {}",
                err.0, err.0
            );
            print_line();
        });

        wait(None)?;
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
    user_input = user_input.to_lowercase();

    Ok(user_input
        .strip_suffix("\r\n")
        .or(user_input.strip_suffix("\n"))
        .unwrap_or(&user_input)
        .to_string())
}

fn wait(custom_msg: Option<&str>) -> Result<()> {
    if let Some(msg) = custom_msg {
        println!("{}", msg);
    } else {
        println!("Press enter to continue.");
    }
    get_user_input()?;
    Ok(())
}

fn print_line() {
    println!("-----------------------------------------------------------------------------");
}
