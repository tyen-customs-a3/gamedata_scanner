use serde::{Serialize, Deserialize};

/// Configuration for the gamedata scanner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataScannerConfig {
    /// File extensions to scan (default: ["cpp", "hpp"])
    pub file_extensions: Vec<String>,
    /// Maximum number of threads to use for parallel processing
    pub max_threads: usize,
    /// Whether to follow symbolic links
    pub follow_symlinks: bool,
    /// Whether to skip files that have already been processed
    pub use_cache: bool,
}

impl Default for GameDataScannerConfig {
    fn default() -> Self {
        Self {
            file_extensions: vec!["cpp".to_string(), "hpp".to_string()],
            max_threads: num_cpus::get(),
            follow_symlinks: false,
            use_cache: true,
        }
    }
} 