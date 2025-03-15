use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Write};
use std::collections::HashMap;
use std::time::Instant;
use parser_code::parse_file;
use serde::{Serialize, Deserialize};
use log::{info, error, warn, debug};
use env_logger::Env;
use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory containing HPP/CPP/EXT files to parse
    #[arg(short, long)]
    input_dir: String,

    /// Directory to copy failing files to
    #[arg(short, long)]
    output_dir: String,

    /// Path to save the JSON report
    #[arg(short, long, default_value = "parse_report.json")]
    report_path: String,

    /// Verbose output
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
    
    /// Maximum number of files to process (for testing)
    #[arg(short, long)]
    max_files: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParseReport {
    total_files: usize,
    successful_files: usize,
    failed_files: usize,
    failures: HashMap<String, String>,
}

fn main() -> io::Result<()> {
    // Initialize the logger with more detailed output
    let env = Env::default()
        .filter_or("RUST_LOG", "info");
    env_logger::Builder::from_env(env)
        .format_timestamp_millis()
        .init();

    // Parse command line arguments
    let args = Args::parse();
    
    // Print startup information
    info!("Batch Parser Tool Starting");
    info!("Input directory: {}", args.input_dir);
    info!("Output directory: {}", args.output_dir);
    info!("Report path: {}", args.report_path);
    info!("Verbosity level: {}", args.verbose);
    if let Some(max) = args.max_files {
        info!("Maximum files to process: {}", max);
    }

    // Verify input directory exists
    if !Path::new(&args.input_dir).exists() {
        error!("Input directory does not exist: {}", args.input_dir);
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Input directory not found: {}", args.input_dir),
        ));
    }

    // Set log level based on verbosity
    match args.verbose {
        0 => log::set_max_level(log::LevelFilter::Info),
        1 => log::set_max_level(log::LevelFilter::Debug),
        _ => log::set_max_level(log::LevelFilter::Trace),
    }

    // Create output directory if it doesn't exist
    info!("Creating output directory: {}", args.output_dir);
    match fs::create_dir_all(&args.output_dir) {
        Ok(_) => info!("Output directory created or already exists"),
        Err(e) => {
            error!("Failed to create output directory: {}", e);
            return Err(e);
        }
    }

    // Initialize report
    let mut report = ParseReport {
        total_files: 0,
        successful_files: 0,
        failed_files: 0,
        failures: HashMap::new(),
    };

    // Start timer
    let start_time = Instant::now();
    info!("Starting batch parsing of files in {}", args.input_dir);

    // Process files
    match process_directory(
        Path::new(&args.input_dir),
        Path::new(&args.output_dir),
        &mut report,
        args.max_files,
    ) {
        Ok(_) => info!("Directory processing completed successfully"),
        Err(e) => {
            error!("Error processing directory: {}", e);
            // Continue to save the report even if processing failed
        }
    }

    // Calculate elapsed time
    let elapsed = start_time.elapsed();
    info!(
        "Parsing completed in {:.2} seconds",
        elapsed.as_secs_f64()
    );

    // Print summary
    info!(
        "Summary: Total: {}, Successful: {}, Failed: {}",
        report.total_files, report.successful_files, report.failed_files
    );

    // Save report to JSON file
    info!("Saving report to {}", args.report_path);
    let report_json = match serde_json::to_string_pretty(&report) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize report to JSON: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    };
    
    match File::create(&args.report_path) {
        Ok(mut file) => {
            match file.write_all(report_json.as_bytes()) {
                Ok(_) => info!("Report saved to {}", args.report_path),
                Err(e) => {
                    error!("Failed to write report to file: {}", e);
                    return Err(e);
                }
            }
        },
        Err(e) => {
            error!("Failed to create report file: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn process_directory(
    input_dir: &Path,
    output_dir: &Path,
    report: &mut ParseReport,
    max_files: Option<usize>,
) -> io::Result<()> {
    if !input_dir.is_dir() {
        error!("Directory not found: {}", input_dir.display());
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", input_dir.display()),
        ));
    }

    info!("Processing directory: {}", input_dir.display());
    
    // Track total files processed across all directories
    let mut total_processed = 0;
    
    // Read directory entries
    let entries = match fs::read_dir(input_dir) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to read directory {}: {}", input_dir.display(), e);
            return Err(e);
        }
    };

    // Process each entry
    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(e) => {
                error!("Failed to read directory entry: {}", e);
                continue;
            }
        };
        
        let path = entry.path();
        
        // Check if we've reached the maximum number of files
        if let Some(max) = max_files {
            if report.total_files >= max {
                info!("Reached maximum number of files to process ({})", max);
                break;
            }
        }

        if path.is_dir() {
            // Recursively process subdirectories
            debug!("Found subdirectory: {}", path.display());
            match process_directory(&path, output_dir, report, max_files) {
                Ok(_) => {},
                Err(e) => {
                    warn!("Error processing subdirectory {}: {}", path.display(), e);
                    // Continue processing other entries
                }
            }
        } else if is_target_file(&path) {
            match process_file(&path, output_dir, report) {
                Ok(_) => {},
                Err(e) => {
                    error!("Error processing file {}: {}", path.display(), e);
                    // Continue processing other files
                }
            }
            
            total_processed += 1;
            if total_processed % 100 == 0 {
                info!("Processed {} files so far", total_processed);
            }
        }
    }

    Ok(())
}

fn is_target_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        return ext == "hpp" || ext == "cpp" || ext == "ext";
    }
    false
}

fn process_file(file_path: &Path, output_dir: &Path, report: &mut ParseReport) -> io::Result<()> {
    report.total_files += 1;
    
    let relative_path = file_path.to_string_lossy().to_string();
    debug!("Processing file: {}", relative_path);

    // Try to parse the file
    match parse_file(file_path) {
        Ok(_) => {
            report.successful_files += 1;
            debug!("Successfully parsed: {}", relative_path);
        }
        Err(errors) => {
            // Check if all errors are warnings - if so, consider it a partial success
            let all_warnings = errors.iter().all(|e| e.severity() == hemtt_workspace::reporting::Severity::Warning);
            
            if all_warnings {
                report.successful_files += 1;
                info!("Partially parsed with warnings: {}", relative_path);
                
                // Store warnings in report for future reference
                let warnings_msg = errors
                    .iter()
                    .map(|e| format!("{}: {}", e.ident(), e.message()))
                    .collect::<Vec<_>>()
                    .join("; ");
                
                report.failures.insert(relative_path.clone(), format!("WARN: {}", warnings_msg));
            } else {
                report.failed_files += 1;
                
                // Create error message
                let error_msg = if errors.is_empty() {
                    "Unknown error".to_string()
                } else {
                    errors
                        .iter()
                        .map(|e| format!("{}: {}", e.ident(), e.message()))
                        .collect::<Vec<_>>()
                        .join("; ")
                };
                
                warn!("Failed to parse {}: {}", relative_path, error_msg);
                
                // Store error in report
                report.failures.insert(relative_path.clone(), error_msg);
                
                // Copy the failing file to the output directory
                // Create a path in the output directory that preserves the structure
                let file_name = file_path.file_name().unwrap_or_default();
                
                let mut output_path = output_dir.to_path_buf();
                output_path.push(file_name);
                
                // Create parent directories
                if let Some(parent) = output_path.parent() {
                    match fs::create_dir_all(parent) {
                        Ok(_) => debug!("Created directory: {}", parent.display()),
                        Err(e) => {
                            error!("Failed to create directory {}: {}", parent.display(), e);
                            return Err(e);
                        }
                    }
                }
                
                // Copy the file
                info!("Copying file from {} to {}", file_path.display(), output_path.display());
                match fs::copy(file_path, &output_path) {
                    Ok(_) => info!("Copied failing file to {}", output_path.display()),
                    Err(e) => {
                        error!("Failed to copy file to {}: {}", output_path.display(), e);
                        return Err(e);
                    }
                }
            }
        }
    }

    Ok(())
} 