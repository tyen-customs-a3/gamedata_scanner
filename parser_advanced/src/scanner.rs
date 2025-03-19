use std::path::{Path, PathBuf};
use std::time::Instant;
use models::{Scanner, ScanResult, GameClass};
use crate::parse_file as advanced_parse_file;

/// Advanced scanner implementation
pub struct AdvancedScanner;

impl AdvancedScanner {
    /// Create a new advanced scanner
    pub fn new() -> Self {
        Self
    }
}

impl Scanner for AdvancedScanner {
    fn scan_directory(&self, dir_path: &Path) -> Result<ScanResult, String> {
        // Track scan time if requested
        let start_time = Instant::now();
        
        let mut result = ScanResult::new();
        let mut files_with_errors = 0;
        
        // Collect all files in the directory
        let mut files = Vec::new();
        collect_files_recursive(dir_path, &mut files)?;
        
        result.files_scanned = files.len();
        
        // Process each file
        for file_path in &files {
            match self.parse_file(file_path) {
                Ok(file_classes) => {
                    // Add classes to the map
                    result.add_classes(file_classes);
                },
                Err(_) => {
                    files_with_errors += 1;
                }
            }
        }
        
        result.files_with_errors = files_with_errors;
        
        // Calculate scan time
        result.scan_time_ms = Some(start_time.elapsed().as_millis() as u64);
        
        Ok(result)
    }
    
    fn parse_file(&self, file_path: &Path) -> Result<Vec<GameClass>, String> {
        match advanced_parse_file(file_path) {
            Ok(classes) => Ok(classes),
            Err(errors) => Err(format!("Error parsing file: {:?}", errors))
        }
    }
}

/// Recursively collect files from a directory
fn collect_files_recursive(dir_path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(format!("Directory does not exist: {}", dir_path.display()));
    }
    
    let entries = std::fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;
    
    for entry in entries.flatten() {
        let path = entry.path();
        
        if path.is_dir() {
            collect_files_recursive(&path, files)?;
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if ext_str == "hpp" || ext_str == "cpp" || ext_str == "h" || ext_str == "c" {
                files.push(path);
            }
        }
    }
    
    Ok(())
} 