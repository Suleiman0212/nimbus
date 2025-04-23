use thiserror::Error;

#[derive(Error, Debug)]
pub enum MessageError {
    #[error("Data encode error")]
    EncodeError,

    #[error("Data decode error")]
    DecodeError,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("No data received, maybe connection was break")]
    NoDataRecieved,
}
