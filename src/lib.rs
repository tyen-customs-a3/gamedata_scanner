//! Game Data Scanner Library
//! 
//! This library provides functionality for scanning and parsing game data files in parallel.
//! It can process multiple files simultaneously and extract class definitions and their properties.
//! 
//! # Example
//! ```no_run
//! use gamedata_scanner::{Scanner, ScannerConfig};
//! 
//! // Create a scanner with default configuration
//! let scanner = Scanner::new(ScannerConfig::default());
//! 
//! // Scan a directory
//! match scanner.scan_directory("path/to/game/data") {
//!     Ok(result) => {
//!         println!("Processed {} files", result.total_files);
//!         println!("Successful: {}", result.successful_files);
//!         println!("Failed: {}", result.failed_files);
//!         
//!         // Process results
//!         for (path, scan_result) in result.results {
//!             println!("File: {}", path.display());
//!             println!("Classes found: {}", scan_result.classes.len());
//!         }
//!     },
//!     Err(e) => eprintln!("Error scanning directory: {}", e),
//! }
//! ```

use std::path::Path;

// Re-export models from models
pub use models::{
    GameClass, ClassProperty, PropertyValue, Scanner as ClassScanner, 
    ScanResult, FileParser
};
pub use scanner_factory::{get_scanner, get_parser};
pub use scanner::ScannerConfig;
pub use scanner::ScannerResult;

pub mod scanner_factory;
pub mod scanner;

/// Main scanner interface for processing game data files
#[derive(Debug, Clone)]
pub struct Scanner {
    config: ScannerConfig,
}

impl Scanner {
    /// Creates a new scanner with the specified configuration
    pub fn new(config: ScannerConfig) -> Self {
        Self { config }
    }

    /// Creates a new scanner with default configuration
    pub fn default() -> Self {
        Self::new(ScannerConfig::default())
    }

    /// Scans a directory recursively for game data files
    /// 
    /// # Arguments
    /// 
    /// * `path` - Path to the directory to scan
    /// 
    /// # Returns
    /// 
    /// Returns a `Result` containing either a `ScannerResult` with the scan results
    /// or an IO error if the scanning process failed.
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use gamedata_scanner::Scanner;
    /// 
    /// let scanner = Scanner::default();
    /// let results = scanner.scan_directory("path/to/data").unwrap();
    /// println!("Found {} classes", results.results.values()
    ///     .map(|r| r.classes.len())
    ///     .sum::<usize>());
    /// ```
    pub fn scan_directory<P: AsRef<Path>>(&self, path: P) -> std::io::Result<ScannerResult> {
        scanner::scan_directory(path, self.config.clone())
    }

    /// Gets a reference to the scanner's configuration
    pub fn config(&self) -> &ScannerConfig {
        &self.config
    }

    /// Gets a mutable reference to the scanner's configuration
    pub fn config_mut(&mut self) -> &mut ScannerConfig {
        &mut self.config
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new(ScannerConfig::default())
    }
}

/// Convenience function to scan a directory with default configuration
pub fn scan_directory<P: AsRef<Path>>(path: P) -> std::io::Result<ScannerResult> {
    Scanner::default().scan_directory(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::PathBuf;

    fn create_test_file(dir: &Path, name: &str, content: &str) -> io::Result<PathBuf> {
        let path = dir.join(name);
        let mut file = File::create(&path)?;
        file.write_all(content.as_bytes())?;
        Ok(path)
    }

    #[test]
    fn test_scanner_api() -> io::Result<()> {
        let temp_dir = TempDir::new()?;
        
        // Create test files
        create_test_file(&temp_dir.path(), "test1.hpp", r#"
            class TestClass1 {
                displayName = "Test1";
            };
        "#)?;

        // Test default scanner
        let scanner = Scanner::default();
        let result = scanner.scan_directory(temp_dir.path())?;
        assert_eq!(result.total_files, 1);
        assert_eq!(result.successful_files, 1);

        // Test custom configuration
        let mut config = ScannerConfig::default();
        config.show_progress = false;
        config.timeout = 60; // Custom timeout
        let scanner = Scanner::new(config);
        let result = scanner.scan_directory(temp_dir.path())?;
        assert_eq!(result.total_files, 1);

        // Test timeout configuration
        let mut config = ScannerConfig::default();
        config.timeout = 15; // Different timeout
        let scanner = Scanner::new(config);
        let result = scanner.scan_directory(temp_dir.path())?;
        assert_eq!(result.total_files, 1);
        
        // Test convenience function
        let result = scan_directory(temp_dir.path())?;
        assert_eq!(result.total_files, 1);

        Ok(())
    }
}
