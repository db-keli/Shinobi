// src/errors.rs
use image::ImageError;
use quircs::{DecodeError, ExtractError};
use serde_json::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ImageReadError {
    IoError(io::Error),
    ImgError(ImageError),
    DecodeError(DecodeError),
    ExtractError(ExtractError),
}

impl From<ImageError> for ImageReadError {
    fn from(err: ImageError) -> Self {
        ImageReadError::ImgError(err)
    }
}

impl From<DecodeError> for ImageReadError {
    fn from(err: DecodeError) -> Self {
        ImageReadError::DecodeError(err)
    }
}

impl From<io::Error> for ImageReadError {
    fn from(err: io::Error) -> Self {
        ImageReadError::IoError(err)
    }
}

impl From<ExtractError> for ImageReadError {
    fn from(err: ExtractError) -> Self {
        ImageReadError::ExtractError(err)
    }
}

impl fmt::Display for ImageReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ImageReadError::IoError(err) => write!(f, "IO error: {}", err),
            ImageReadError::ImgError(err) => write!(f, "Image error: {}", err),
            ImageReadError::DecodeError(err) => write!(f, "Decode error: {}", err),
            ImageReadError::ExtractError(err) => write!(f, "Extract error: {}", err),
        }
    }
}

#[derive(Debug)]
pub enum WorkerError {
    IoError(io::Error),
    CommandFailed(String),
    SerdeError(Error),
}

impl From<io::Error> for WorkerError {
    fn from(err: io::Error) -> Self {
        WorkerError::IoError(err)
    }
}

impl From<Error> for WorkerError {
    fn from(err: Error) -> Self {
        WorkerError::SerdeError(err)
    }
}

impl std::error::Error for WorkerError {}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WorkerError::IoError(err) => write!(f, "IO error: {}", err),
            WorkerError::CommandFailed(err) => write!(f, "Command failed: {}", err),
            WorkerError::SerdeError(err) => write!(f, "Serde error: {}", err),
        }
    }
}
