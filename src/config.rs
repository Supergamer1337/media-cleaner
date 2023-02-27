use std::fs;

use color_eyre::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub overseerr_url: String,
    pub overseerr_token: String,
}

pub fn read_conf() -> Result<Config> {
    let reader = fs::File::open("config.yaml")?;
    let conf: Config = serde_yaml::from_reader(reader)?;
    Ok(conf)
}
