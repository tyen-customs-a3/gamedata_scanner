use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Instant, Duration};
use log::{debug, error, trace, warn};
use codespan_reporting::diagnostic::Severity;
use std::io::Write;
use std::thread;
use std::sync::mpsc;

use crate::parse_file;
use super::error_handler::{categorize_error, extract_related_context};
use super::types::FileFailure;
use super::config::Args;

/// Generate a unique file path by adding a numeric suffix if the file already exists
fn get_unique_path(path: &Path) -> PathBuf {
    if !path.exists() {
        return path.to_path_buf();
    }
    
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("file");
    let extension = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");
    let parent = path.parent().unwrap_or_else(|| Path::new(""));
    
    let mut counter = 1;
    loop {
        let new_name = if extension.is_empty() {
            format!("{}_{}", file_stem, counter)
        } else {
            format!("{}_{}.{}", file_stem, counter, extension)
        };
        
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

/// Handle a failing file by copying it to the failures directory and logging diagnostics
fn handle_failing_file(
    file_path: &Path,
    failure: &FileFailure,
    output_dir: &Path,
) -> io::Result<()> {
    // Create failures directory if it doesn't exist
    let failures_dir = output_dir.join("failures");
    fs::create_dir_all(&failures_dir)?;
    
    // Create category subdirectory
    let category_dir = failures_dir.join(&failure.error_category.to_lowercase().replace(' ', "_"));
    fs::create_dir_all(&category_dir)?;
    
    // Copy the file to the category directory with a unique name
    let file_name = file_path.file_name().unwrap_or_default();
    let initial_dest_path = category_dir.join(file_name);
    let dest_path = get_unique_path(&initial_dest_path);
    fs::copy(file_path, &dest_path)?;
    
    // Create a diagnostic file next to the copied file with a matching name
    let diag_path = dest_path.with_extension("diag.txt");
    let mut diag_file = fs::File::create(&diag_path)?;
    
    // Write diagnostic information
    writeln!(diag_file, "Error Category: {}", failure.error_category)?;
    writeln!(diag_file, "Error Severity: {}", failure.error_severity)?;
    writeln!(diag_file, "Error Message: {}", failure.error_message)?;
    writeln!(diag_file, "File Size: {} bytes", failure.file_size)?;
    writeln!(diag_file, "Processing Time: {}ms", failure.parse_duration_ms)?;
    writeln!(diag_file, "Original File: {}", file_path.display())?;
    
    if let Some(line) = failure.error_line_number {
        writeln!(diag_file, "Error Line: {}", line)?;
    }
    
    if !failure.diagnostics.is_empty() {
        writeln!(diag_file, "\nDiagnostic Messages:")?;
        for diag in &failure.diagnostics {
            writeln!(diag_file, "  - {}", diag)?;
        }
    }
    
    if let Some(context) = &failure.error_context {
        writeln!(diag_file, "\nError Context:\n{}", context)?;
    }
    
    debug!("Copied failing file to: {}", dest_path.display());
    debug!("Created diagnostic file: {}", diag_path.display());
    
    Ok(())
}

/// Process a single file with timeout and return a tuple containing:
/// - A boolean indicating success (true) or failure (false)
/// - An option containing failure details (None if successful)
pub fn process_file(file_path: &Path, output_dir: Option<&Path>, args: &Args) -> (bool, Option<FileFailure>) {
    let file_path_str = file_path.to_string_lossy().to_string();
    debug!("Processing file: {}", file_path_str);
    
    // Get file metadata
    let file_size = match fs::metadata(file_path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            error!("Failed to get file size for {}: {}", file_path_str, e);
            return create_io_error_failure(&e, file_path);
        }
    };
    
    // Record start time for performance metrics
    let start_time = Instant::now();
    
    // Validate file size
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
    if file_size > MAX_FILE_SIZE {
        warn!("File {} is too large ({} bytes), skipping", file_path_str, file_size);
        let failure = FileFailure {
            error_message: format!("File too large: {} bytes (max: {} bytes)", file_size, MAX_FILE_SIZE),
            diagnostics: vec![],
            is_timeout: false,
            file_size,
            error_line_number: None,
            error_context: None,
            error_category: "Size Error".to_string(),
            error_severity: "Error".to_string(),
            related_files: vec![file_path_str],
            parse_duration_ms: 0,
        };
        
        if let Some(output_dir) = output_dir {
            if let Err(e) = handle_failing_file(file_path, &failure, output_dir) {
                error!("Failed to handle failing file: {}", e);
            }
        }
        
        return (false, Some(failure));
    }
    
    // Create channel for result communication
    let (tx, rx) = mpsc::channel();
    let file_path_thread = file_path.to_path_buf();
    
    // Spawn processing in a separate thread
    thread::spawn(move || {
        let result = parse_file(&file_path_thread);
        let _ = tx.send(result); // Ignore send error as receiver might be dropped on timeout
    });
    
    // Wait for the thread with timeout
    let timeout = if args.timeout_secs > 0 {
        Duration::from_secs(args.timeout_secs)
    } else {
        Duration::from_secs(u64::MAX) // Effectively unlimited
    };
    
    let parse_result = match rx.recv_timeout(timeout) {
        Ok(result) => result,
        Err(_) => {
            // Handle timeout
            let duration = start_time.elapsed().as_millis() as u64;
            let failure = FileFailure {
                error_message: format!("Processing timeout after {}ms (limit: {}s)", 
                    duration, args.timeout_secs),
                diagnostics: vec![],
                is_timeout: true,
                file_size,
                error_line_number: None,
                error_context: None,
                error_category: "Timeout Error".to_string(),
                error_severity: "Error".to_string(),
                related_files: vec![file_path_str.clone()],
                parse_duration_ms: duration,
            };
            
            if let Some(output_dir) = output_dir {
                if let Err(e) = handle_failing_file(file_path, &failure, output_dir) {
                    error!("Failed to handle failing file: {}", e);
                }
            }
            
            warn!("File {} processing timed out after {}ms (limit: {}s)", 
                file_path_str, duration, args.timeout_secs);
            return (false, Some(failure));
        }
    };
    
    let duration = start_time.elapsed().as_millis() as u64;
    
    match parse_result {
        Ok(classes) => {
            debug!("Successfully parsed {} classes from {}", classes.len(), file_path_str);
            trace!("Parsing took {}ms", duration);
            (true, None)
        },
        Err(errors) => {
            // Check if all errors are warnings
            let all_warnings = errors.iter().all(|e| format!("{:?}", e.severity()) == "Warning");
            
            // Extract diagnostic information
            let mut diagnostics = Vec::new();
            let mut error_line_number = None;
            let mut related_files = vec![file_path_str.clone()];
            
            for error in &errors {
                if let Some(diagnostic) = error.diagnostic() {
                    // Extract error line number if available
                    if error_line_number.is_none() {
                        if let Some(label) = diagnostic.labels.first() {
                            if let Ok(content) = label.file().read_to_string() {
                                let lines = content.lines().count();
                                error_line_number = Some(lines);
                            }
                        }
                    }
                    
                    // Collect related files from the labels
                    for label in &diagnostic.labels {
                        let related_path = label.file().as_str();
                        if !related_files.contains(&related_path.to_string()) {
                            related_files.push(related_path.to_string());
                        }
                    }
                    
                    // Add diagnostic to the list
                    diagnostics.push(error.message());
                }
            }
            
            // Create a summary error message
            let error_message = if errors.is_empty() {
                "Unknown error".to_string()
            } else {
                errors
                    .iter()
                    .map(|e| format!("{}: {}", e.ident(), e.message()))
                    .collect::<Vec<_>>()
                    .join("; ")
            };
            
            // Extract error context including related files
            let error_context = extract_related_context(file_path, &related_files, error_line_number);
            
            // Categorize the error
            let error_category = categorize_error(&error_message);
            
            // Determine error severity
            let error_severity = if all_warnings {
                "Warning"
            } else {
                "Error"
            }.to_string();
            
            let failure = FileFailure {
                error_message,
                diagnostics,
                is_timeout: false,
                file_size,
                error_line_number,
                error_context,
                error_category,
                error_severity: error_severity.clone(),
                related_files,
                parse_duration_ms: duration,
            };
            
            // Handle failing file if output directory is provided and it's an error
            if !all_warnings {
                if let Some(output_dir) = output_dir {
                    if let Err(e) = handle_failing_file(file_path, &failure, output_dir) {
                        error!("Failed to handle failing file: {}", e);
                    }
                }
            }
            
            debug!("File {} processed with {}: {}", 
                   file_path_str, 
                   error_severity.to_lowercase(), 
                   failure.error_message);
            
            // Return the processed result
            (all_warnings, Some(failure))
        }
    }
}

/// Create a failure result for IO errors
fn create_io_error_failure(error: &io::Error, file_path: &Path) -> (bool, Option<FileFailure>) {
    let file_path_str = file_path.to_string_lossy().to_string();
    (false, Some(FileFailure {
        error_message: format!("Failed to read file: {}", error),
        diagnostics: vec![],
        is_timeout: false,
        file_size: 0,
        error_line_number: None,
        error_context: None,
        error_category: "IO Error".to_string(),
        error_severity: "Error".to_string(),
        related_files: vec![file_path_str],
        parse_duration_ms: 0,
    }))
}

/// Utility function to copy a file to the output directory
pub fn copy_file_to_output(source: &Path, dest_dir: &Path) -> io::Result<()> {
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }
    
    let file_name = source
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid source path"))?;
    
    let initial_dest_path = dest_dir.join(file_name);
    let dest_path = get_unique_path(&initial_dest_path);
    
    debug!("Copying {} to {}", source.display(), dest_path.display());
    fs::copy(source, dest_path)?;
    
    Ok(())
} 