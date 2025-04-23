#![allow(unused)]

use std::{fs::File, io::Read};

use crate::error::ConfigError;
use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub server_addres: String,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub password: String,
}

type ConfigResult = Result<Config, ConfigError>;

impl Config {
    pub fn get() -> ConfigResult {
        let path = config_dir()
            .ok_or(ConfigError::UnableToGetDir)?
            .join("nimbus/config.ron");
        let mut buf = Default::default();
        File::open(&path)?.read_to_string(&mut buf)?;
        let config: Config = ron::from_str(&buf)?;
        Ok(config)
    }
}
