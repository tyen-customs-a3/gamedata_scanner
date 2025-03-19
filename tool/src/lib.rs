pub mod config;

use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::sync::atomic::{AtomicUsize, Ordering};

use log::{debug, info, warn, error};
use chrono::{DateTime, Local};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::{Serialize, Deserialize};

// Use the gamedata_scanner library as the entry point to scanner functionality
use gamedata_scanner::{ClassScanner, get_scanner, GameClass};
use models::ScanResult;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScanSummary {
    pub start_time: DateTime<Local>,
    pub end_time: DateTime<Local>,
    pub duration_seconds: f64,
    pub files_scanned: usize,
    pub files_processed: usize,
    pub files_failed: usize,
    pub classes_found: usize,
    pub parser_used: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub parent: Option<String>,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanReport {
    pub summary: ScanSummary,
    pub classes: Vec<ClassInfo>,
}

pub fn run(args: config::Args) -> Result<(), String> {
    // Set up logging
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .init();
    
    let start_time = Local::now();
    
    // Determine whether to use advanced parser from parser_type
    let use_advanced = match args.parser_type {
        config::ParserType::Advanced => true,
        config::ParserType::Simple => false,
    };
    
    info!("Starting batch parsing with {} parser", args.parser_type);
    info!("Input directory: {}", args.input_dir.display());
    
    // Ensure output directories exist
    fs::create_dir_all(&args.output_dir).map_err(|e| format!("Failed to create output directory: {}", e))?;
    
    // Get list of files to process
    let files = collect_files(&args.input_dir, &args.file_extensions, args.max_files)?;
    info!("Found {} files to process", files.len());
    
    let progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    
    // Track progress and results
    let classes_found = Arc::new(Mutex::new(Vec::new()));
    let files_failed = Arc::new(Mutex::new(Vec::new()));
    let files_processed = Arc::new(AtomicUsize::new(0));
    let fail_counter = Arc::new(AtomicUsize::new(0));
    
    // Get the appropriate scanner from the main library
    let scanner = get_scanner(use_advanced);
    
    // Process files
    let result = if args.parallel {
        // Create a list of file tasks with their own scanner instances
        let result: Result<(), String> = files.par_iter().try_for_each(|file_path| {
            // Create a fresh scanner for each thread
            let thread_scanner = get_scanner(use_advanced);
            
            // Process the file
            match process_single_file(file_path, &thread_scanner) {
                Ok(file_classes) => {
                    // Store class information
                    let mut classes = classes_found.lock().unwrap();
                    for class in file_classes {
                        classes.push(ClassInfo {
                            name: class.name,
                            parent: class.parent,
                            file: file_path.to_string_lossy().to_string(),
                        });
                    }
                    
                    let processed_count = files_processed.fetch_add(1, Ordering::SeqCst) + 1;
                    if processed_count % 10 == 0 {
                        progress_bar.set_message(format!("Found {} classes so far", classes.len()));
                    }
                },
                Err(e) => {
                    // Track failed files
                    let mut failed = files_failed.lock().unwrap();
                    failed.push(file_path.clone());
                    
                    // Copy failed file if requested
                    if args.copy_failed_files {
                        let file_name = file_path.file_name().unwrap_or_default();
                        let dest_path = args.output_dir.join(file_name);
                        if let Err(e) = fs::copy(file_path, &dest_path) {
                            warn!("Failed to copy failed file: {}", e);
                        }
                    }
                    
                    let fail_count = fail_counter.fetch_add(1, Ordering::SeqCst) + 1;
                    progress_bar.set_message(format!("Failed files: {}", fail_count));
                    
                    // Check if we've exceeded max failures
                    if fail_count >= args.max_failures {
                        warn!("Maximum failure count reached ({}), stopping scan", args.max_failures);
                        return Err(format!("Maximum failure count reached ({})", args.max_failures));
                    }

                    // Increment files_processed counter for failed files too
                    files_processed.fetch_add(1, Ordering::SeqCst);
                }
            }
            
            progress_bar.inc(1);
            Ok(())
        });
        
        result
    } else {
        // Sequential processing
        let result: Result<(), String> = files.iter().try_for_each(|file_path| {
            // Process the file
            match process_single_file(file_path, &scanner) {
                Ok(file_classes) => {
                    // Store class information
                    let mut classes = classes_found.lock().unwrap();
                    for class in file_classes {
                        classes.push(ClassInfo {
                            name: class.name,
                            parent: class.parent,
                            file: file_path.to_string_lossy().to_string(),
                        });
                    }
                    
                    let processed_count = files_processed.fetch_add(1, Ordering::SeqCst) + 1;
                    progress_bar.set_message(format!("Found {} classes so far", classes.len()));
                },
                Err(e) => {
                    // Track failed files
                    let mut failed = files_failed.lock().unwrap();
                    failed.push(file_path.clone());
                    
                    // Copy failed file if requested
                    if args.copy_failed_files {
                        let file_name = file_path.file_name().unwrap_or_default();
                        let dest_path = args.output_dir.join(file_name);
                        if let Err(e) = fs::copy(file_path, &dest_path) {
                            warn!("Failed to copy failed file: {}", e);
                        }
                    }
                    
                    let fail_count = fail_counter.fetch_add(1, Ordering::SeqCst) + 1;
                    progress_bar.set_message(format!("Failed files: {}", fail_count));
                    
                    // Check if we've exceeded max failures
                    if fail_count >= args.max_failures {
                        warn!("Maximum failure count reached ({}), stopping scan", args.max_failures);
                        return Err(format!("Maximum failure count reached ({})", args.max_failures));
                    }

                    // Increment files_processed counter for failed files too
                    files_processed.fetch_add(1, Ordering::SeqCst);
                }
            }
            
            progress_bar.inc(1);
            Ok(())
        });
        
        result
    };
    
    progress_bar.finish_with_message("Scan complete");
    
    // Prepare report
    let end_time = Local::now();
    let duration = end_time.signed_duration_since(start_time);
    
    let all_classes = classes_found.lock().unwrap();
    let failed = files_failed.lock().unwrap();
    
    let summary = ScanSummary {
        start_time,
        end_time,
        duration_seconds: duration.num_milliseconds() as f64 / 1000.0,
        files_scanned: files.len(),
        files_processed: files_processed.load(Ordering::SeqCst),
        files_failed: failed.len(),
        classes_found: all_classes.len(),
        parser_used: args.parser_type.to_string(),
    };
    
    let report = ScanReport {
        summary: summary.clone(),
        classes: all_classes.clone(),
    };
    
    // Write report to JSON
    let json = serde_json::to_string_pretty(&report)
        .map_err(|e| format!("Failed to serialize report: {}", e))?;
    
    let mut file = File::create(&args.report_path)
        .map_err(|e| format!("Failed to create report file: {}", e))?;
    
    file.write_all(json.as_bytes())
        .map_err(|e| format!("Failed to write report: {}", e))?;
    
    // Log summary
    info!("Scan complete:");
    info!("  Parser used: {}", args.parser_type);
    info!("  Files processed: {}/{}", files_processed.load(Ordering::SeqCst), files.len());
    info!("  Files failed: {}", failed.len());
    info!("  Classes found: {}", all_classes.len());
    info!("  Time taken: {:.2} seconds", summary.duration_seconds);
    info!("  Report saved to: {}", args.report_path.display());
    
    // Return result based on whether the scan was interrupted
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

fn collect_files(dir_path: &Path, extensions: &str, max_files: usize) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    let exts: Vec<&str> = extensions.split(',').collect();
    
    collect_files_recursive(dir_path, &mut files, &exts, max_files)?;
    
    Ok(files)
}

fn collect_files_recursive(dir_path: &Path, files: &mut Vec<PathBuf>, extensions: &[&str], max_files: usize) -> Result<(), String> {
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(format!("Directory does not exist: {}", dir_path.display()));
    }
    
    let entries = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {}: {}", dir_path.display(), e))?;
    
    for entry in entries.flatten() {
        let path = entry.path();
        
        if files.len() >= max_files {
            break;
        }
        
        if path.is_dir() {
            collect_files_recursive(&path, files, extensions, max_files)?;
        } else if let Some(ext) = path.extension() {
            let ext_str = ext.to_string_lossy().to_lowercase();
            if extensions.contains(&ext_str.as_str()) {
                files.push(path);
            }
        }
    }
    
    Ok(())
}

/// Process a single file using the scanner from the main library
fn process_single_file(file_path: &Path, scanner: &Arc<dyn ClassScanner>) -> Result<Vec<ClassInfo>, String> {
    debug!("Processing file: {}", file_path.display());
    
    let start = Instant::now();
    
    // Use the scanner to parse the file
    match scanner.parse_file(file_path) {
        Ok(parsed_classes) => {
            let classes = parsed_classes.into_iter()
                .map(|class| ClassInfo {
                    name: class.name,
                    parent: class.parent,
                    file: file_path.to_string_lossy().to_string(),
                })
                .collect::<Vec<ClassInfo>>();
            
            let duration = start.elapsed();
            debug!("Processed {} in {:.2?}, found {} classes", 
                   file_path.display(), duration, classes.len());
            
            Ok(classes)
        },
        Err(e) => Err(format!("Failed to parse file {}: {}", file_path.display(), e))
    }
} 