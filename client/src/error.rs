use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    Config(#[from] config::error::ConfigError),

    #[error(transparent)]
    Protocol(#[from] protocol::error::MessageError),

    #[error(transparent)]
    FileSys(#[from] filesys::error::FsError),

    #[error("Received invalid argemts")]
    InvalidArguments,

    #[error("Received incorrect message")]
    IncorrectMessage,

    #[error("User data isn't correct, unsuccessful login")]
    IncorrectUser,

    #[error("Error received: {0}")]
    ErrReceived(String),

    #[error("Session isn't exists")]
    UnexistsSession,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
