use serde::{Serialize, Deserialize};

/// Configuration for the gamedata scanner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDataScannerConfig {
    /// File extensions to scan (default: ["cpp", "hpp"])
    pub file_extensions: Vec<String>,
    /// Maximum number of threads to use for parallel processing
    pub max_threads: usize,
    /// Maximum number of files to scan (None = no limit)
    pub max_files: Option<usize>,
}

impl Default for GameDataScannerConfig {
    fn default() -> Self {
        Self {
            file_extensions: vec!["cpp".into(), "hpp".into()],
            max_threads: num_cpus::get().max(1), // Ensure at least 1 thread
            max_files: None,
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
    
    /// Create a new configuration with a maximum file limit
    pub fn with_max_files(max_files: usize) -> Self {
        let mut config = Self::default();
        config.max_files = Some(max_files);
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
        self.config.max_threads = threads;
        self
    }
    
    /// Set the maximum number of files to scan
    pub fn with_max_files(mut self, max_files: Option<usize>) -> Self {
        self.config.max_files = max_files;
        self
    }
    
    /// Build the configuration
    pub fn build(self) -> GameDataScannerConfig {
        self.config
    }
} 