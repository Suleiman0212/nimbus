use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClientError {
    #[error(transparent)]
    Config(#[from] config::error::ConfigError),

    #[error(transparent)]
    Protocol(#[from] protocol::error::MessageError),

    #[error("Received incorrect message")]
    IncorrectMessage,

    #[error("User data isn't correct, unsuccessful login")]
    IncorrectUser,

    #[error("Session isn't exists")]
    UnexistsSession,

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
