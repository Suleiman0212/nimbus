use dirs::config_dir;
use fs;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Read, Write};
use toml;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub server_ip: String,
    pub user: User,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub pass: String,
}

pub fn get_config() -> Result<Config, Box<dyn Error>> {
    let path = get_config_path()?;

    let mut toml = fs::load_file(&path, "config.toml")?;
    let mut buf: Vec<u8> = Vec::new();
    toml.read_to_end(&mut buf)?;
    let toml = String::from_utf8_lossy(&buf);
    let config: Config = toml::from_str(&toml)?;
    Ok(config)
}

pub fn crete_conf() -> Result<(), Box<dyn Error>> {
    let path = get_config_path()?;
    fs::create_dir(&path)?;
    let config: Config = Config {
        server_ip: "127.0.0.1:8080".to_string(),
        user: User {
            name: "Alice".to_string(),
            pass: "AliceTheBest123".to_string(),
        },
    };

    let toml = toml::to_string_pretty(&config)?;
    let mut file = fs::create_file(&path, "config.toml")?;
    file.write_all(toml.as_bytes())?;
    Ok(())
}

pub fn get_config_path() -> Result<String, Box<dyn Error>> {
    let config_dir = config_dir();
    let path = match config_dir {
        Some(d) => d,
        None => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "InvalidType: file name incorrect type (isnt Text).",
            )))
        }
    };
    let mut path = path.to_string_lossy().into_owned();
    path.push_str("/nimbus/");
    Ok(path)
}