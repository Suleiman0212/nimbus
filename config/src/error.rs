use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Unable to get config directory")]
    UnableToGetDir,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Spanned(#[from] ron::error::SpannedError),
}
