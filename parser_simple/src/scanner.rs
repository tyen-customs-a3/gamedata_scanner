use std::path::Path;
use std::time::Instant;
use gamedata_scanner_models::{Scanner, ScanResult, GameClass};
use crate::SimpleClassScanner;

/// Simple scanner implementation 
pub struct SimpleScanner;

impl SimpleScanner {
    /// Create a new simple scanner
    pub fn new() -> Self {
        Self
    }
}

impl Scanner for SimpleScanner {
    fn scan_directory(&self, dir_path: &Path) -> Result<ScanResult, String> {
        // Track scan time if requested
        let start_time = Instant::now();
        
        // Use the simple scanner
        let scanner = SimpleClassScanner::new();
        let classes = scanner.scan_directory(dir_path);
        
        let mut result = ScanResult::new();
        
        // Update scan stats - count unique files
        let mut unique_files = std::collections::HashSet::new();
        for class in &classes {
            unique_files.insert(class.file_path.to_string_lossy().to_string());
        }
        result.files_scanned = unique_files.len();
        
        // Add all classes to result
        result.add_classes(classes);
        
        // Calculate scan time
        result.scan_time_ms = Some(start_time.elapsed().as_millis() as u64);
        
        Ok(result)
    }
    
    fn parse_file(&self, file_path: &Path) -> Result<Vec<GameClass>, String> {
        let scanner = SimpleClassScanner::new();
        let classes = scanner.scan_file(file_path);
        
        Ok(classes)
    }
} 