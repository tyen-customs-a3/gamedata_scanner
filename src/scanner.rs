use std::path::{Path, PathBuf};
use std::time::Instant;
use parser_code::{parse_file, CodeClass};
use rayon::prelude::*;
use log::{info, warn, debug, error};
use walkdir::WalkDir;

use crate::config::GameDataScannerConfig;
use crate::models::{FileResult, ScanResult, ClassMap};
use crate::error::{ScanError, ScanResult as Result};

/// Scan a directory for game data files and parse them
///
/// # Arguments
///
/// * `directory` - The directory to scan (Path, not PathBuf for flexibility)
/// * `config` - Configuration options for the scanner
///
/// # Returns
///
/// * `Result<ScanResult>` - The scan results or an error
///
/// # Errors
///
/// Returns `ScanError::NoFilesFound` if no files matching the extensions were found
/// Returns `ScanError::IoError` if there was an error reading the directory
pub async fn scan_directory(
    directory: &Path, 
    config: &GameDataScannerConfig
) -> Result<ScanResult> {
    info!("Starting scan of directory: {}", directory.display());
    let start_time = Instant::now();
    
    // Find all files with the specified extensions using walkdir
    let files = find_files(directory, config)?;
    
    if files.is_empty() {
        return Err(ScanError::NoFilesFound(directory.to_path_buf()));
    }
    
    info!("Found {} files to scan", files.len());
    
    // Process files in parallel using Rayon
    let file_results: Vec<_> = files
        .par_iter()
        .map(|file| scan_file(file))
        .collect();
    
    // Create the ScanResult
    let mut scan_result = ScanResult::new(directory);
    scan_result.total_scan_time_ms = start_time.elapsed().as_millis() as u64;
    
    // Process the file results
    for file_result in file_results {
        scan_result.add_file_result(file_result);
    }
    
    info!("Scan completed in {}ms", scan_result.total_scan_time_ms);
    info!("Scanned {} files, found {} classes", scan_result.files_scanned, scan_result.classes_found);
    
    if scan_result.files_with_errors > 0 {
        warn!("{} files had errors during scanning", scan_result.files_with_errors);
    }
    
    Ok(scan_result)
}

/// Find all files with the specified extensions in a directory
/// 
/// Uses walkdir for efficient directory traversal
fn find_files(
    directory: &Path, 
    config: &GameDataScannerConfig
) -> Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = WalkDir::new(directory)
        .into_iter()
        .filter_map(|entry| {
            match entry {
                Ok(entry) => {
                    // Only process files
                    if !entry.file_type().is_file() {
                        return None;
                    }
                    
                    // Check if the file has a valid extension
                    if let Some(ext) = entry.path().extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if config.file_extensions.iter().any(|valid| valid.to_lowercase() == ext_str) {
                            debug!("Found file: {}", entry.path().display());
                            return Some(entry.path().to_path_buf());
                        }
                    }
                    
                    None
                },
                Err(e) => {
                    warn!("Error accessing path: {}", e);
                    None
                }
            }
        })
        .collect();
    
    // Sort the files to ensure deterministic results
    files.sort();
    
    // Apply max_files limit if specified in config
    if let Some(max_files) = config.max_files {
        if files.len() > max_files {
            info!("Limiting scan to {} files out of {} found", max_files, files.len());
            files.truncate(max_files);
        }
    }
    
    if files.is_empty() {
        warn!("No files with extensions {:?} found in {}", config.file_extensions, directory.display());
    }
    
    Ok(files)
}

/// Scan a single file and return the results
fn scan_file(file_path: &Path) -> FileResult {
    debug!("Scanning file: {}", file_path.display());
    let start_time = Instant::now();
    
    let mut result = FileResult::new(file_path);
    
    match parse_file(file_path) {
        Ok(classes) => {
            debug!("Found {} classes in {}", classes.len(), file_path.display());
            result.classes = classes;
        },
        Err(errors) => {
            let error_msg = format!("Failed to parse file: {:?}", errors);
            error!("{} - {}", file_path.display(), error_msg);
            result.add_error(error_msg);
        }
    }
    
    result.scan_time_ms = start_time.elapsed().as_millis() as u64;
    result
}

/// Get a filtered class map containing only classes that match a predicate
pub fn filter_classes<P>(
    class_map: &ClassMap,
    predicate: P
) -> ClassMap
where
    P: Fn(&CodeClass) -> bool + Sync + Send
{
    class_map.iter()
        .filter_map(|(name, classes)| {
            let filtered: Vec<CodeClass> = classes.iter()
                .filter(|class| predicate(class))
                .cloned()
                .collect();
            
            if filtered.is_empty() {
                None
            } else {
                Some((name.clone(), filtered))
            }
        })
        .collect()
} 