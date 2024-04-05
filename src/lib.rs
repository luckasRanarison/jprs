mod utils;

pub mod kanjidic2;

use quick_xml::DeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error when performing I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid file")]
    InvalidFile,
    #[error("Failed to parse file: {0}")]
    Parse(#[from] DeError),
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
