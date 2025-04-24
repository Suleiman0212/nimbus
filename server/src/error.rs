use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    Config(#[from] config::error::ConfigError),

    #[error(transparent)]
    Protocol(#[from] protocol::error::MessageError),

    #[error(transparent)]
    FileSys(#[from] filesys::error::FsError),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
