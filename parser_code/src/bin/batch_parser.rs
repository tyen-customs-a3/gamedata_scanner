use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::{self, Write};
use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex, atomic::{AtomicUsize, Ordering}};
use parser_code::parse_file;
use serde::{Serialize, Deserialize};
use log::{info, error, warn, debug, trace};
use env_logger::Env;
use clap::{Parser, ArgAction};
use chrono::{DateTime, Local};
use hemtt_workspace::reporting::WorkspaceFiles;
use hemtt_workspace::WorkspacePath;
use regex::Regex;
use pathdiff;
use rayon::prelude::*;

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

    /// Path to save the diagnostic report
    #[arg(short = 'd', long, default_value = "diagnostic_report.log")]
    diagnostic_path: String,

    /// Verbose output
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
    
    /// Maximum number of files to process (for testing)
    #[arg(short, long)]
    max_files: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ParseReport {
    timestamp: String,
    total_files: usize,
    successful_files: usize,
    failed_files: usize,
    failures: HashMap<String, FileFailure>,
}

#[derive(Debug, Serialize, Deserialize)]
struct FileFailure {
    error_message: String,
    diagnostics: Vec<String>,
}

/// Strip ANSI color codes from a string
fn strip_ansi_codes(input: &str) -> String {
    // This regex matches standard ANSI color and control codes
    lazy_static::lazy_static! {
        static ref ANSI_REGEX: Regex = Regex::new(r"\x1b\[[0-9;]*[a-zA-Z]").unwrap();
    }
    ANSI_REGEX.replace_all(input, "").to_string()
}

/// Write a diagnostic report with detailed error information and line context for each failing file
fn write_diagnostic_report(report: &ParseReport, diagnostic_path: &str) -> io::Result<()> {
    let mut file = File::create(diagnostic_path)?;
    
    writeln!(file, "Diagnostic Report - Generated at: {}", report.timestamp)?;
    writeln!(file, "Total files processed: {}", report.total_files)?;
    writeln!(file, "Successful files: {}", report.successful_files)?;
    writeln!(file, "Failed files: {}", report.failed_files)?;
    writeln!(file, "\n=== Detailed Diagnostics ===\n")?;
    
    for (file_path, failure) in &report.failures {
        writeln!(file, "File: {}", file_path)?;
        writeln!(file, "Error: {}", failure.error_message)?;
        
        if !failure.diagnostics.is_empty() {
            writeln!(file, "Detailed Diagnostics:")?;
            for (i, diagnostic) in failure.diagnostics.iter().enumerate() {
                // Strip ANSI color codes before writing to file
                let clean_diagnostic = strip_ansi_codes(diagnostic);
                writeln!(file, "Diagnostic #{}: ", i + 1)?;
                writeln!(file, "{}", clean_diagnostic)?;
                writeln!(file)?; // Add a blank line between diagnostics
            }
        } else {
            writeln!(file, "No detailed diagnostics available")?;
        }
        writeln!(file, "\n{}", "-".repeat(80))?;
        writeln!(file)?;
    }
    
    Ok(())
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
    debug!("Input directory: {}", args.input_dir);
    debug!("Output directory: {}", args.output_dir);
    debug!("Report path: {}", args.report_path);
    debug!("Diagnostic path: {}", args.diagnostic_path);
    debug!("Verbosity level: {}", args.verbose);
    if let Some(max) = args.max_files {
        debug!("Maximum files to process: {}", max);
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
    debug!("Creating output directory: {}", args.output_dir);
    match fs::create_dir_all(&args.output_dir) {
        Ok(_) => debug!("Output directory created or already exists"),
        Err(e) => {
            error!("Failed to create output directory: {}", e);
            return Err(e);
        }
    }

    // Initialize report
    let report = Arc::new(Mutex::new(ParseReport {
        timestamp: Local::now().to_rfc3939(),
        total_files: 0,
        successful_files: 0,
        failed_files: 0,
        failures: HashMap::new(),
    }));

    // Start timer
    let start_time = Instant::now();
    
    // Scan all files upfront
    info!("Scanning for files in {}", args.input_dir);
    let files_to_process = match scan_files(
        Path::new(&args.input_dir),
        args.max_files,
    ) {
        Ok(files) => {
            info!("Found {} files to process", files.len());
            files
        },
        Err(e) => {
            error!("Error scanning directory: {}", e);
            return Err(e);
        }
    };
    
    // Update total files count
    {
        let mut report_guard = report.lock().unwrap();
        report_guard.total_files = files_to_process.len();
    }
    
    // Create a progress counter
    let processed_count = Arc::new(AtomicUsize::new(0));
    
    // Process files in parallel
    info!("Starting batch parsing of {} files using multiple threads", files_to_process.len());
    files_to_process.par_iter().for_each(|file_path| {
        if let Err(e) = process_file(
            file_path, 
            Path::new(&args.input_dir), 
            Path::new(&args.output_dir), 
            Arc::clone(&report)
        ) {
            error!("Error processing file {}: {}", file_path.display(), e);
        }
        
        // Update progress counter
        let count = processed_count.fetch_add(1, Ordering::SeqCst) + 1;
        if count % 100 == 0 || count == files_to_process.len() {
            info!("Processed {} of {} files", count, files_to_process.len());
        }
    });

    // Get the final report 
    let final_report = Arc::try_unwrap(report)
        .expect("Still references to report")
        .into_inner()
        .expect("Report mutex poisoned");

    // Calculate elapsed time
    let elapsed = start_time.elapsed();
    info!(
        "Parsing completed in {:.2} seconds",
        elapsed.as_secs_f64()
    );

    // Print summary
    info!(
        "Summary: Total: {}, Successful: {}, Failed: {}",
        final_report.total_files, final_report.successful_files, final_report.failed_files
    );

    // Save report to JSON file
    debug!("Saving report to {}", args.report_path);
    let report_json = match serde_json::to_string_pretty(&final_report) {
        Ok(json) => json,
        Err(e) => {
            error!("Failed to serialize report to JSON: {}", e);
            return Err(io::Error::new(io::ErrorKind::Other, e));
        }
    };
    
    match File::create(&args.report_path) {
        Ok(mut file) => {
            match file.write_all(report_json.as_bytes()) {
                Ok(_) => debug!("Report saved to {}", args.report_path),
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

    // Write diagnostic report
    debug!("Writing diagnostic report to {}", args.diagnostic_path);
    match write_diagnostic_report(&final_report, &args.diagnostic_path) {
        Ok(_) => debug!("Diagnostic report saved to {}", args.diagnostic_path),
        Err(e) => {
            error!("Failed to write diagnostic report: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

/// Scan the directory and build a list of all files to process
fn scan_files(
    root_dir: &Path,
    max_files: Option<usize>,
) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    scan_directory_recursive(root_dir, &mut files, max_files)?;
    Ok(files)
}

/// Recursively scan a directory and collect all target files
fn scan_directory_recursive(
    current_dir: &Path,
    files: &mut Vec<PathBuf>,
    max_files: Option<usize>,
) -> io::Result<()> {
    if !current_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", current_dir.display()),
        ));
    }

    debug!("Scanning directory: {}", current_dir.display());
    
    let entries = match fs::read_dir(current_dir) {
        Ok(entries) => entries,
        Err(e) => {
            error!("Failed to read directory {}: {}", current_dir.display(), e);
            return Err(e);
        }
    };

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
            if files.len() >= max {
                debug!("Reached maximum number of files to scan ({})", max);
                break;
            }
        }

        if path.is_dir() {
            trace!("Found subdirectory: {}", path.display());
            if let Err(e) = scan_directory_recursive(&path, files, max_files) {
                warn!("Error scanning subdirectory {}: {}", path.display(), e);
            }
        } else if is_target_file(&path) {
            trace!("Found file to process: {}", path.display());
            files.push(path);
            
            if files.len() % 1000 == 0 {
                info!("Found {} files so far", files.len());
            }
        }
    }

    Ok(())
}

fn process_file(
    file_path: &Path,
    input_dir: &Path,
    output_dir: &Path,
    report: Arc<Mutex<ParseReport>>,
) -> io::Result<()> {
    let relative_path = file_path.to_string_lossy().to_string();
    trace!("Processing file: {}", relative_path);

    // Create a workspace path for the file
    let workspace_path = WorkspacePath::slim_file(file_path).map_err(|e| {
        io::Error::new(io::ErrorKind::Other, format!("Failed to create workspace path: {}", e))
    })?;

    // Create workspace files for diagnostic formatting
    let workspace_files = WorkspaceFiles::new();

    // Try to parse the file
    match parse_file(file_path) {
        Ok(_) => {
            // Update the report with successful parse
            let mut report_guard = report.lock().unwrap();
            report_guard.successful_files += 1;
            trace!("Successfully parsed: {}", relative_path);
        }
        Err(errors) => {
            // Check if all errors are warnings - if so, consider it a partial success
            let all_warnings = errors.iter().all(|e| e.severity() == hemtt_workspace::reporting::Severity::Warning);
            
            // Collect detailed diagnostic information with full context
            let diagnostics: Vec<String> = errors.iter()
                .filter_map(|e| {
                    e.diagnostic().map(|d| {
                        let mut d = d.clone();
                        // Store the message before moving d
                        let message = d.message.clone();
                        // Add the workspace path to the diagnostic if it has no labels
                        if d.labels.is_empty() {
                            d = d.with_label(
                                hemtt_workspace::reporting::Label::primary(workspace_path.clone(), 0..0)
                                    .with_message(message)
                            );
                        }
                        // Format diagnostic with workspace context for source lines
                        let diagnostic_str = d.to_string(&workspace_files);
                        trace!("Diagnostic for {}: {}", relative_path, diagnostic_str);
                        diagnostic_str
                    })
                })
                .collect();
            
            // Create a summary error message
            let error_summary = if errors.is_empty() {
                "Unknown error".to_string()
            } else {
                errors
                    .iter()
                    .map(|e| format!("{}: {}", e.ident(), e.message()))
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            
            // Update the report with results
            let mut report_guard = report.lock().unwrap();
            
            if all_warnings {
                report_guard.successful_files += 1;
                debug!("Partially parsed with warnings: {}", relative_path);
                
                report_guard.failures.insert(relative_path.clone(), FileFailure {
                    error_message: format!("WARN: {}", error_summary),
                    diagnostics,
                });
            } else {
                report_guard.failed_files += 1;
                
                debug!("Failed to parse {}: {}", relative_path, error_summary);
                
                // Store error in report with detailed diagnostics
                report_guard.failures.insert(relative_path.clone(), FileFailure {
                    error_message: error_summary,
                    diagnostics,
                });
                
                // Determine the relative path structure to preserve
                let rel_path = match pathdiff::diff_paths(file_path, input_dir) {
                    Some(path) => path,
                    None => PathBuf::from(file_path.file_name().unwrap_or_default()),
                };
                
                // Create full output path that preserves the directory structure
                let mut output_path = output_dir.to_path_buf();
                output_path.push(&rel_path);
                
                // Create parent directories
                if let Some(parent) = output_path.parent() {
                    match fs::create_dir_all(parent) {
                        Ok(_) => trace!("Created directory: {}", parent.display()),
                        Err(e) => {
                            error!("Failed to create directory {}: {}", parent.display(), e);
                            return Err(e);
                        }
                    }
                }
                
                // Copy the file
                debug!("Copying file from {} to {}", file_path.display(), output_path.display());
                match fs::copy(file_path, &output_path) {
                    Ok(_) => debug!("Copied failing file to {}", output_path.display()),
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

fn is_target_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        let ext = extension.to_string_lossy().to_lowercase();
        return ext == "hpp" || ext == "cpp" || ext == "ext";
    }
    false
} 