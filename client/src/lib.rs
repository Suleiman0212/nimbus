use config::client::Config;
use error::ClientError;
use once_cell::sync::Lazy;

pub mod error;
pub mod processor;

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::get().unwrap());
pub type ClientHandle = Result<(), ClientError>;
