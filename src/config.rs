use color_eyre::Result;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub items_shown: usize,
    pub plex: Plex,
    pub overseerr: Overseerr,
    pub tautulli: Tautulli,
    pub sonarr: Option<Sonarr>,
    pub radarr: Option<Radarr>,
}

#[derive(Debug, Deserialize)]
pub struct Plex {
    pub url: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct Overseerr {
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Tautulli {
    pub url: String,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Sonarr {
    pub api_key: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Radarr {
    pub api_key: String,
    pub url: String,
}

static INSTANCE: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("Config has not been initialized.")
    }

    pub fn read_conf() -> Result<()> {
        let reader = fs::File::open("config.yaml")?;
        let conf: Config = serde_yaml::from_reader(reader)?;
        INSTANCE
            .set(conf)
            .expect("Config has already been initialized.");
        Ok(())
    }
}
