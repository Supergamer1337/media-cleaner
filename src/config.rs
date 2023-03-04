use std::fs;

use color_eyre::Result;
use once_cell::sync::OnceCell;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub overseerr: Overseerr,
    pub tautulli: Tautulli,
    pub tmdb: TMDB,
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
pub struct TMDB {
    pub v3_key: String,
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
