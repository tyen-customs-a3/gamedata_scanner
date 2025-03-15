use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during scanning
#[derive(Error, Debug)]
pub enum ScanError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Failed to scan file {0}: {1}")]
    FileScanError(PathBuf, String),
    
    #[error("No files found to scan in {0}")]
    NoFilesFound(PathBuf),
} 