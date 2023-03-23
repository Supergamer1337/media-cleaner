use color_eyre::Result;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::fs;

static INSTANCE: OnceCell<Config> = OnceCell::new();
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_items_shown")]
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

impl Config {
    pub fn global() -> &'static Config {
        INSTANCE.get().expect("Config has not been initialized.")
    }

    pub fn read_conf() -> Result<()> {
        if let Some(_) = INSTANCE.get() {
            return Ok(());
        }

        let reader = fs::File::open("config.yaml")?;
        let mut conf: Config = serde_yaml::from_reader(reader)?;

        Self::remove_trailing_slashes(&mut conf);

        INSTANCE
            .set(conf)
            .expect("Config has already been initialized.");
        Ok(())
    }

    fn remove_trailing_slashes(conf: &mut Config) {
        remove_trailing_slash(&mut conf.overseerr.url);
        remove_trailing_slash(&mut conf.plex.url);
        remove_trailing_slash(&mut conf.tautulli.url);

        if let Some(ref mut radarr) = conf.radarr {
            remove_trailing_slash(&mut radarr.url);
        }

        if let Some(ref mut sonarr) = conf.sonarr {
            remove_trailing_slash(&mut sonarr.url);
        }
    }
}

fn default_items_shown() -> usize {
    5
}

fn remove_trailing_slash(url: &mut String) {
    if url.ends_with("/") {
        url.pop();
    }
}
