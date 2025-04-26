use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
