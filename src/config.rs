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
    pub sonarr_4k: Option<Sonarr>,
    pub radarr: Option<Radarr>,
    pub radarr_4k: Option<Radarr>,
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

        Self::clean_urls(&mut conf);

        INSTANCE
            .set(conf)
            .expect("Config has already been initialized.");
        Ok(())
    }

    fn clean_urls(conf: &mut Config) {
        clean_url(&mut conf.overseerr.url);
        clean_url(&mut conf.plex.url);
        clean_url(&mut conf.tautulli.url);

        if let Some(ref mut radarr) = conf.radarr {
            clean_url(&mut radarr.url);
        }

        if let Some(ref mut radarr) = conf.radarr_4k {
            clean_url(&mut radarr.url);
        }

        if let Some(ref mut sonarr) = conf.sonarr {
            clean_url(&mut sonarr.url);
        }

        if let Some(ref mut sonarr) = conf.sonarr_4k {
            clean_url(&mut sonarr.url);
        }
    }
}

fn default_items_shown() -> usize {
    5
}

fn clean_url(url: &mut String) {
    if url.ends_with("/") {
        url.pop();
    }
}
