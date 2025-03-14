use std::path::{Path, PathBuf};
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashMap;
use parser_code::parse_file;
use rayon::prelude::*;
use log::{info, warn, debug, error};

use crate::config::GameDataScannerConfig;
use crate::models::{FileResult, ScanResult};
use crate::error::ScanError;

/// Scan a directory for game data files and parse them
pub async fn scan_directory(
    directory: &Path, 
    config: &GameDataScannerConfig
) -> Result<ScanResult, ScanError> {
    info!("Starting scan of directory: {}", directory.display());
    let start_time = Instant::now();
    
    // Find all files with the specified extensions
    let files = find_files(directory, &config.file_extensions, config.follow_symlinks)?;
    
    if files.is_empty() {
        return Err(ScanError::NoFilesFound(directory.to_path_buf()));
    }
    
    info!("Found {} files to scan", files.len());
    
    // Set up thread pool
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.max_threads)
        .build()
        .unwrap();
    
    // Shared results that will be updated by multiple threads
    let file_results = Arc::new(Mutex::new(Vec::with_capacity(files.len())));
    
    // Process files in parallel
    pool.install(|| {
        files.par_iter().for_each(|file| {
            let file_result = scan_file(file);
            let mut results = file_results.lock().unwrap();
            results.push(file_result);
        });
    });
    
    // Collect results
    let file_results = Arc::try_unwrap(file_results)
        .expect("Thread pool should be done with file_results")
        .into_inner()
        .unwrap();
    
    // Build class map
    let mut class_map = HashMap::new();
    let mut classes_found = 0;
    let mut files_with_errors = 0;
    
    for result in &file_results {
        classes_found += result.classes.len();
        
        if !result.errors.is_empty() {
            files_with_errors += 1;
        }
        
        for class in &result.classes {
            class_map
                .entry(class.name.clone())
                .or_insert_with(Vec::new)
                .push(class.clone());
        }
    }
    
    let total_scan_time_ms = start_time.elapsed().as_millis() as u64;
    let files_scanned = file_results.len();
    
    info!("Scan completed in {}ms", total_scan_time_ms);
    info!("Scanned {} files, found {} classes", files_scanned, classes_found);
    
    if files_with_errors > 0 {
        warn!("{} files had errors during scanning", files_with_errors);
    }
    
    Ok(ScanResult {
        root_directory: directory.to_path_buf(),
        file_results,
        class_map,
        total_scan_time_ms,
        files_scanned,
        classes_found,
        files_with_errors,
    })
}

/// Find all files with the specified extensions in a directory
fn find_files(
    directory: &Path, 
    extensions: &[String], 
    follow_symlinks: bool
) -> Result<Vec<PathBuf>, ScanError> {
    let mut files = Vec::new();
    find_files_recursive(directory, extensions, follow_symlinks, &mut files)?;
    Ok(files)
}

/// Recursively find all files with the specified extensions
fn find_files_recursive(
    directory: &Path, 
    extensions: &[String], 
    follow_symlinks: bool, 
    files: &mut Vec<PathBuf>
) -> Result<(), ScanError> {
    if !directory.is_dir() {
        return Ok(());
    }
    
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            // Skip symlinks if not following them
            if path.is_symlink() && !follow_symlinks {
                continue;
            }
            
            find_files_recursive(&path, extensions, follow_symlinks, files)?;
        } else if let Some(extension) = path.extension() {
            let extension = extension.to_string_lossy().to_lowercase();
            
            if extensions.iter().any(|ext| ext.to_lowercase() == extension) {
                files.push(path);
            }
        }
    }
    
    Ok(())
}

/// Scan a single file and return the results
fn scan_file(file_path: &Path) -> FileResult {
    debug!("Scanning file: {}", file_path.display());
    let start_time = Instant::now();
    
    let mut result = FileResult {
        file_path: file_path.to_path_buf(),
        classes: Vec::new(),
        scan_time_ms: 0,
        errors: Vec::new(),
    };
    
    match parse_file(file_path) {
        Ok(classes) => {
            debug!("Found {} classes in {}", classes.len(), file_path.display());
            result.classes = classes;
        },
        Err(errors) => {
            let error_msg = format!("Failed to parse file: {:?}", errors);
            error!("{} - {}", file_path.display(), error_msg);
            result.errors.push(error_msg);
        }
    }
    
    result.scan_time_ms = start_time.elapsed().as_millis() as u64;
    result
} 