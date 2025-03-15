use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur during scanning
#[derive(Error, Debug)]
pub enum ScanError {
    /// IO error from the standard library
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    /// Error that occurred when scanning a specific file
    #[error("Failed to scan file {0}: {1}")]
    FileScanError(PathBuf, String),
    
    /// No files were found to scan in the specified directory
    #[error("No files found to scan in {0}")]
    NoFilesFound(PathBuf),
    
    /// Parser error when parsing a file
    #[error("Parser error in {0}: {1}")]
    ParserError(PathBuf, String),
    
    /// An error occurred during parallel execution
    #[error("Thread pool error: {0}")]
    ThreadPoolError(String),
    
    /// Failed to create thread pool
    #[error("Failed to create thread pool: {0}")]
    ThreadPoolCreationError(#[from] rayon::ThreadPoolBuildError),
}

/// Result type for scan operations
pub type ScanResult<T> = std::result::Result<T, ScanError>;

/// Helper methods for working with scan errors
impl ScanError {
    /// Create a new FileScanError
    pub fn file_scan_error(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::FileScanError(path.into(), message.into())
    }
    
    /// Create a new ParserError
    pub fn parser_error(path: impl Into<PathBuf>, message: impl Into<String>) -> Self {
        Self::ParserError(path.into(), message.into())
    }
} 