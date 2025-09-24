use thiserror::Error;

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("File not found")]
    FileNotFound,
    #[error("Invalid format")]
    InvalidFormat,
}
