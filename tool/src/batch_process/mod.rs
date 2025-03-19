use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use log::{debug, error, info, warn};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::atomic::{AtomicUsize, AtomicBool};
use std::sync::atomic::Ordering;

pub mod config;
mod error_handler;
mod processor;
mod reporter;
mod scanner;
mod types;

pub use config::Args;
use processor::process_file;
use reporter::generate_report;
use scanner::scan_directory;
use types::{FileFailure, Report};

/// Run the batch parser with the given arguments
pub fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    args.setup_logging();
    
    let start_time = Instant::now();
    info!("Starting batch parser");
    
    // Validate arguments
    args.validate()?;
    
    // Create output directory if it doesn't exist
    if !args.output_dir.exists() {
        fs::create_dir_all(&args.output_dir)?;
        debug!("Created output directory: {}", args.output_dir.display());
    }
    
    // Scan for files
    info!("Scanning directory: {}", args.input_dir.display());
    let files = scan_directory(&args.input_dir, &args.file_extensions, Some(args.max_files));
    
    if files.is_empty() {
        warn!("No files found to process");
        return Ok(());
    }
    
    info!("Found {} files to process", files.len());
    
    // Process files
    let report = if args.parallel {
        info!("Processing files in parallel");
        process_files_parallel(&files, &args)
    } else {
        info!("Processing files sequentially");
        process_files_sequential(&files, &args)
    };
    
    // Generate reports
    info!("Generating reports in: {}", args.output_dir.display());
    generate_report(&args.output_dir, &report)?;
    
    let duration = start_time.elapsed();
    info!("Batch processing completed in {:.2}s", duration.as_secs_f32());
    
    // Print summary statistics
    print_summary_stats(&report);
    
    Ok(())
}

fn process_files_parallel(files: &[PathBuf], args: &Args) -> Report {
    let report = Arc::new(Mutex::new(Report::new()));
    
    // Create progress bar
    let progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
        .unwrap()
        .progress_chars("##-"));
    
    // Create atomic counter for failures
    let failure_count = Arc::new(AtomicUsize::new(0));
    let should_stop = Arc::new(AtomicBool::new(false));
    
    files.par_iter().for_each(|file| {
        // Check if we should stop due to too many failures
        if args.max_failures > 0 && failure_count.load(Ordering::SeqCst) >= args.max_failures {
            should_stop.store(true, Ordering::SeqCst);
            return;
        }
        
        if should_stop.load(Ordering::SeqCst) {
            return;
        }
        
        let (success, failure) = process_file(file, Some(&args.output_dir), args);
        let mut report = report.lock().unwrap();
        
        if success {
            report.successful_files.push(file.to_string_lossy().to_string());
        }
        if let Some(ref err) = failure {
            report.failures.push(err.clone());
            if err.error_severity == "Error" {
                failure_count.fetch_add(1, Ordering::SeqCst);
            }
            progress_bar.set_message(format!("Last: {} ({})", 
                file.file_name().unwrap_or_default().to_string_lossy(),
                err.error_category));
        } else {
            progress_bar.set_message(format!("Last: {}", 
                file.file_name().unwrap_or_default().to_string_lossy()));
        }
        
        progress_bar.inc(1);
    });
    
    let stopped_early = should_stop.load(Ordering::SeqCst);
    progress_bar.finish_with_message(if stopped_early {
        "Processing stopped due to too many failures"
    } else {
        "Processing complete"
    });
    
    let mut report = Arc::try_unwrap(report).unwrap().into_inner().unwrap();
    report.update_stats();
    report.stopped_early = stopped_early;
    report
}

fn process_files_sequential(files: &[PathBuf], args: &Args) -> Report {
    let mut report = Report::new();
    let mut failure_count = 0;
    
    // Create progress bar
    let progress_bar = ProgressBar::new(files.len() as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
        .unwrap()
        .progress_chars("##-"));
    
    for file in files {
        // Check if we should stop due to too many failures
        if args.max_failures > 0 && failure_count >= args.max_failures {
            report.stopped_early = true;
            break;
        }
        
        let (success, failure) = process_file(file, Some(&args.output_dir), args);
        
        if success {
            report.successful_files.push(file.to_string_lossy().to_string());
        }
        if let Some(ref err) = failure {
            report.failures.push(err.clone());
            if err.error_severity == "Error" {
                failure_count += 1;
            }
            progress_bar.set_message(format!("Last: {} ({})", 
                file.file_name().unwrap_or_default().to_string_lossy(),
                err.error_category));
        } else {
            progress_bar.set_message(format!("Last: {}", 
                file.file_name().unwrap_or_default().to_string_lossy()));
        }
        
        progress_bar.inc(1);
    }
    
    progress_bar.finish_with_message(if report.stopped_early {
        "Processing stopped due to too many failures"
    } else {
        "Processing complete"
    });
    
    report.update_stats();
    report
}

fn print_summary_stats(report: &Report) {
    info!("\nProcessing Summary:");
    info!("  Total files processed: {}", report.stats.total_files);
    info!("  Successfully processed: {}", report.stats.successful_files);
    info!("  Failed with errors: {}", report.stats.failed_files);
    info!("  Files with warnings: {}", report.stats.warning_files);
    info!("  Timeouts: {}", report.stats.timeout_files);
    
    if !report.failures.is_empty() {
        info!("\nError Categories:");
        let mut categories = std::collections::HashMap::new();
        for failure in &report.failures {
            *categories.entry(failure.error_category.clone()).or_insert(0) += 1;
        }
        
        for (category, count) in categories {
            info!("  {}: {}", category, count);
        }
    }
} 