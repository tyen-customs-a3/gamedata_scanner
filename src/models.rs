use std::path::PathBuf;
use std::collections::HashMap;
use std::fmt;
use parser_code::CodeClass;
use serde::{Serialize, Deserialize};

/// Type alias for a map of class names to their definitions
pub type ClassMap = HashMap<String, Vec<CodeClass>>;

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

impl Default for FileResult {
    fn default() -> Self {
        Self {
            file_path: PathBuf::new(),
            classes: Vec::new(),
            scan_time_ms: 0,
            errors: Vec::new(),
        }
    }
}

impl FileResult {
    /// Create a new FileResult for the given path
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
            ..Default::default()
        }
    }
    
    /// Check if the scan had any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Add a class to the scan results
    pub fn add_class(&mut self, class: CodeClass) {
        self.classes.push(class);
    }
    
    /// Add an error to the scan results
    pub fn add_error(&mut self, error: impl Into<String>) {
        self.errors.push(error.into());
    }
}

impl fmt::Display for FileResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileResult {{ path: {}, classes: {}, errors: {} }}", 
            self.file_path.display(), self.classes.len(), self.errors.len())
    }
}

/// Result of scanning a directory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    /// Root directory that was scanned
    pub root_directory: PathBuf,
    /// Results for each file that was scanned
    pub file_results: Vec<FileResult>,
    /// Map of class names to their definitions
    pub class_map: ClassMap,
    /// Total time taken to scan all files in milliseconds
    pub total_scan_time_ms: u64,
    /// Number of files scanned
    pub files_scanned: usize,
    /// Number of classes found
    pub classes_found: usize,
    /// Number of files with errors
    pub files_with_errors: usize,
}

impl ScanResult {
    /// Create a new scan result for the given directory
    pub fn new(root_directory: impl Into<PathBuf>) -> Self {
        Self {
            root_directory: root_directory.into(),
            file_results: Vec::new(),
            class_map: ClassMap::new(),
            total_scan_time_ms: 0,
            files_scanned: 0,
            classes_found: 0,
            files_with_errors: 0,
        }
    }
    
    /// Add a file result to the scan
    pub fn add_file_result(&mut self, result: FileResult) {
        self.files_scanned += 1;
        self.classes_found += result.classes.len();
        
        if result.has_errors() {
            self.files_with_errors += 1;
        }
        
        // Add each class to the class map
        for class in &result.classes {
            self.class_map
                .entry(class.name.clone())
                .or_insert_with(Vec::new)
                .push(class.clone());
        }
        
        self.file_results.push(result);
    }
}

impl fmt::Display for ScanResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ScanResult {{ files: {}, classes: {}, errors: {} }}", 
            self.files_scanned, self.classes_found, self.files_with_errors)
    }
} 