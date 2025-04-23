use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    Config(#[from] config::error::ConfigError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
