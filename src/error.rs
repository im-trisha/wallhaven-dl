use std::{fmt::Display, io, str::Utf8Error};

use glob::{GlobError, PatternError};
use image::ImageError;

pub type Result<T> = core::result::Result<T, Error>;

type ShellexpandError = shellexpand::LookupError<std::env::VarError>;
#[derive(Debug)]
pub enum Error {
    InvalidFirefoxPath, // This should never happen
    InvalidHomePath,    // This should never happen pt2
    VarLookup(ShellexpandError),
    InvalidUtf8(Utf8Error),
    IO(io::Error),
    InvalidGlob(PatternError),
    SerDeError(serde_json::Error),
    ErrorGlobbing(GlobError),
    ImageProcessing(ImageError),
    WallhavenError(wallhaven_api::Error),
}
pub struct WallpaperError {
    pub id: String,
    pub variant: Error,
}

impl WallpaperError {
    pub fn from<T>(id: &str, error: T) -> Self
    where
        Error: From<T>,
    {
        Self {
            id: id.to_string(),
            variant: Error::from(error),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFirefoxPath => write!(f, "Invalid firefox path. How did this happen"),
            Self::InvalidHomePath => write!(f, "Invalid home path. How did this happen?"),
            Self::IO(err) => write!(f, "Input/Output error. {err} "),
            Self::InvalidGlob(err) => write!(f, "Invalid glob. Programmer retarded. {err} "),
            Self::SerDeError(err) => write!(f, "Ser/De error (invalid .jsonlz4?): {err}"),
            Self::ErrorGlobbing(err) => write!(f, "Error globbing. Check file permission? {err}"),
            Self::ImageProcessing(err) => write!(f, "Error processing image: {err}"),
            Self::WallhavenError(err) => write!(f, "Wallhaven error: {err}"),
            Self::InvalidUtf8(err) => write!(f, "Invalid utf8. You're racist {err}"),
            Self::VarLookup(err) => write!(f, "Error in var-lookup: {err}"),
        }
    }
}

impl Display for WallpaperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wallpaper {} - {}", self.id, self.variant)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<PatternError> for Error {
    fn from(value: PatternError) -> Self {
        Self::InvalidGlob(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::SerDeError(value)
    }
}

impl From<GlobError> for Error {
    fn from(value: GlobError) -> Self {
        Self::ErrorGlobbing(value)
    }
}

impl From<wallhaven_api::Error> for Error {
    fn from(value: wallhaven_api::Error) -> Self {
        Self::WallhavenError(value)
    }
}

impl From<ImageError> for Error {
    fn from(value: ImageError) -> Self {
        Self::ImageProcessing(value)
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidUtf8(value)
    }
}

impl From<ShellexpandError> for Error {
    fn from(value: ShellexpandError) -> Self {
        Self::VarLookup(value)
    }
}
