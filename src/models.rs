use std::path::PathBuf;
use std::collections::HashMap;
use parser_code::CodeClass;
use serde::{Serialize, Deserialize};

/// Result of scanning a single file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResult {
    /// Path to the file that was scanned
    pub file_path: PathBuf,
    /// Classes found in the file
    pub classes: Vec<CodeClass>,
    /// Time taken to scan the file in milliseconds
    pub scan_time_ms: u64,
    /// Any errors that occurred during scanning
    pub errors: Vec<String>,
}

/// Result of scanning a directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Root directory that was scanned
    pub root_directory: PathBuf,
    /// Results for each file that was scanned
    pub file_results: Vec<FileResult>,
    /// Map of class names to their definitions
    pub class_map: HashMap<String, Vec<CodeClass>>,
    /// Total time taken to scan all files in milliseconds
    pub total_scan_time_ms: u64,
    /// Number of files scanned
    pub files_scanned: usize,
    /// Number of classes found
    pub classes_found: usize,
    /// Number of files with errors
    pub files_with_errors: usize,
} 