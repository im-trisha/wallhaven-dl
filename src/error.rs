use thiserror::Error;
use tokio::task::JoinError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid firefox path. How did this happen?")]
    InvalidFirefoxPath, // This should never happen
    #[error("Invalid home path. How did this happen?")]
    InvalidHomePath, // This should never happen pt2
    #[error("Error while downloading a chunk of the image.")]
    FileDownloading,
    #[error("Input/Output error. {0}")]
    VarLookup(#[from] shellexpand::LookupError<std::env::VarError>),
    #[error("Invalid glob. Programmer retarded. {0}")]
    InvalidUtf8(#[from] std::str::Utf8Error),
    #[error("Ser/De error (invalid .jsonlz4?): {0}")]
    IO(#[from] std::io::Error),
    #[error("Error globbing. Check file permission? {0}")]
    InvalidGlob(#[from] glob::PatternError),
    #[error("Error deserializing: {0}")]
    SerDeError(#[from] serde_json::Error),
    #[error("Wallhaven error: {0}")]
    ErrorGlobbing(#[from] glob::GlobError),
    #[error("Image error: {0}")]
    ImageProcessing(#[from] image::ImageError),
    #[error("Wallhaven error: {0}")]
    WallhavenError(#[from] wallhaven_rs::Error),
    #[error("Error setting the logger: {0}")]
    LoggerError(#[from] log::SetLoggerError),
    #[error("Join error: {0}")]
    JoinError(#[from] JoinError),
}
