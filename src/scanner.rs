use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use parser_code::{parse_file, CodeClass};
use serde::{Deserialize, Serialize};

/// Result of scanning a single file, containing the parsed classes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileScanResult {
    /// Path to the file that was scanned
    pub file_path: PathBuf,
    /// Classes found in the file
    pub classes: Vec<CodeClass>,
}

/// Configuration for the scanner
#[derive(Debug, Clone)]
pub struct ScannerConfig {
    /// Maximum number of files to process (for testing)
    pub max_files: Option<usize>,
    /// Whether to show progress bar
    pub show_progress: bool,
    /// File extensions to scan (lowercase)
    pub extensions: Vec<String>,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            max_files: None,
            show_progress: true,
            extensions: vec!["hpp".to_string(), "cpp".to_string(), "ext".to_string()],
        }
    }
}

/// Result of the scanning process
#[derive(Debug)]
pub struct ScannerResult {
    /// Total number of files processed
    pub total_files: usize,
    /// Number of successfully processed files
    pub successful_files: usize,
    /// Number of files that failed processing
    pub failed_files: usize,
    /// Map of file paths to their scan results
    pub results: HashMap<PathBuf, FileScanResult>,
    /// Map of file paths to their error messages
    pub errors: HashMap<PathBuf, String>,
}

/// Scans a directory recursively for game data files and processes them in parallel
pub fn scan_directory(
    root_dir: impl AsRef<Path>,
    config: ScannerConfig,
) -> io::Result<ScannerResult> {
    let root_dir = root_dir.as_ref();
    
    // Verify input directory exists
    if !root_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", root_dir.display()),
        ));
    }

    // First, collect all files to process
    let mut files = Vec::new();
    collect_files_recursive(root_dir, &mut files, &config)?;

    // Apply max_files limit if specified
    if let Some(max) = config.max_files {
        files.truncate(max);
    }

    // Create shared result containers
    let results = Arc::new(Mutex::new(HashMap::new()));
    let errors = Arc::new(Mutex::new(HashMap::new()));
    let successful_count = Arc::new(Mutex::new(0usize));
    let failed_count = Arc::new(Mutex::new(0usize));

    // Create progress bar if enabled
    let progress_bar = if config.show_progress {
        let pb = ProgressBar::new(files.len() as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} ({eta}) {msg}")
            .unwrap()
            .progress_chars("##-"));
        Some(pb)
    } else {
        None
    };

    // Process files in parallel
    files.par_iter().for_each(|file_path| {
        match process_file(file_path) {
            Ok(scan_result) => {
                results.lock().unwrap().insert(file_path.clone(), scan_result);
                *successful_count.lock().unwrap() += 1;
            }
            Err(e) => {
                errors.lock().unwrap().insert(file_path.clone(), e.to_string());
                *failed_count.lock().unwrap() += 1;
            }
        }
        
        if let Some(pb) = &progress_bar {
            pb.inc(1);
        }
    });

    // Finish progress bar
    if let Some(pb) = progress_bar {
        pb.finish_with_message("Scan complete");
    }

    // Build final result
    Ok(ScannerResult {
        total_files: files.len(),
        successful_files: *successful_count.lock().unwrap(),
        failed_files: *failed_count.lock().unwrap(),
        results: Arc::try_unwrap(results).unwrap().into_inner().unwrap(),
        errors: Arc::try_unwrap(errors).unwrap().into_inner().unwrap(),
    })
}

/// Recursively collects files to process
fn collect_files_recursive(
    dir: &Path,
    files: &mut Vec<PathBuf>,
    config: &ScannerConfig,
) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_files_recursive(&path, files, config)?;
        } else if is_target_file(&path, &config.extensions) {
            files.push(path);
        }
    }

    Ok(())
}

/// Checks if a file should be processed based on its extension
fn is_target_file(path: &Path, extensions: &[String]) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| extensions.contains(&ext.to_lowercase()))
        .unwrap_or(false)
}

/// Process a single file
fn process_file(file_path: &Path) -> io::Result<FileScanResult> {
    match parse_file(file_path) {
        Ok(classes) => Ok(FileScanResult {
            file_path: file_path.to_path_buf(),
            classes,
        }),
        Err(errors) => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Parse errors: {:?}", errors),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> io::Result<PathBuf> {
        let path = dir.join(name);
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(path)
    }

    #[test]
    fn test_scanner_basic() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        
        // Create test files
        create_test_file(temp_dir.path(), "test1.hpp", r#"
            class TestClass1 {
                displayName = "Test1";
            };
        "#)?;
        
        create_test_file(temp_dir.path(), "test2.cpp", r#"
            class TestClass2 {
                displayName = "Test2";
            };
        "#)?;

        let config = ScannerConfig {
            show_progress: false,
            ..Default::default()
        };

        let result = scan_directory(temp_dir.path(), config)?;

        assert_eq!(result.total_files, 2);
        assert!(result.successful_files > 0);
        
        Ok(())
    }

    #[test]
    fn test_scanner_with_max_files() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        
        // Create multiple test files
        for i in 0..5 {
            create_test_file(temp_dir.path(), &format!("test{}.hpp", i), &format!(r#"
                class TestClass{} {{
                    displayName = "Test{}";
                }};
            "#, i, i))?;
        }

        let config = ScannerConfig {
            max_files: Some(3),
            show_progress: false,
            ..Default::default()
        };

        let result = scan_directory(temp_dir.path(), config)?;

        assert_eq!(result.total_files, 3);
        
        Ok(())
    }
} 