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
            file_extensions: vec!["cpp".into(), "hpp".into()],
            max_threads: num_cpus::get().max(1), // Ensure at least 1 thread
            follow_symlinks: false,
            use_cache: true,
        }
    }
}

impl GameDataScannerConfig {
    /// Create a new builder for configuring the scanner
    pub fn builder() -> GameDataScannerConfigBuilder {
        GameDataScannerConfigBuilder::default()
    }
    
    /// Create a new configuration with the specified file extensions
    pub fn with_extensions(extensions: Vec<String>) -> Self {
        let mut config = Self::default();
        config.file_extensions = extensions;
        config
    }
}

/// Builder for GameDataScannerConfig
#[derive(Debug, Default)]
pub struct GameDataScannerConfigBuilder {
    config: GameDataScannerConfig,
}

impl GameDataScannerConfigBuilder {
    /// Set the file extensions to scan
    pub fn with_extensions(mut self, extensions: Vec<String>) -> Self {
        self.config.file_extensions = extensions;
        self
    }
    
    /// Set the maximum number of threads to use
    pub fn with_max_threads(mut self, threads: usize) -> Self {
        self.config.max_threads = threads.max(1); // Ensure at least 1 thread
        self
    }
    
    /// Set whether to follow symbolic links
    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.config.follow_symlinks = follow;
        self
    }
    
    /// Set whether to use caching
    pub fn use_cache(mut self, use_cache: bool) -> Self {
        self.config.use_cache = use_cache;
        self
    }
    
    /// Build the configuration
    pub fn build(self) -> GameDataScannerConfig {
        self.config
    }
} 