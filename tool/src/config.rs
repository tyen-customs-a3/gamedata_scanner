use clap::{Parser, ValueEnum};
use std::path::PathBuf;
use std::str::FromStr;
use std::fmt;

/// Parser type
#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum ParserType {
    /// The simple regex-based parser
    Simple,
    /// The advanced parser with full property parsing
    Advanced,
}

impl fmt::Display for ParserType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserType::Simple => write!(f, "simple"),
            ParserType::Advanced => write!(f, "advanced"),
        }
    }
}

impl FromStr for ParserType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "simple" => Ok(ParserType::Simple),
            "advanced" => Ok(ParserType::Advanced),
            _ => Err(format!("Unknown parser type: {}", s)),
        }
    }
}

/// Batch parser for Arma 3 game data files
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory containing files to parse
    #[arg(short, long)]
    pub input_dir: PathBuf,

    /// Directory to output failed files to
    #[arg(short, long)]
    pub output_dir: PathBuf,

    /// Path to save the JSON report
    #[arg(short, long)]
    pub report_path: PathBuf,

    /// Path to save diagnostic information
    #[arg(short, long)]
    pub diagnostic_path: PathBuf,

    /// File extensions to parse (comma-separated)
    #[arg(short, long, default_value = "hpp,cpp,h,c")]
    pub file_extensions: String,

    /// Maximum number of files to process
    #[arg(long, default_value_t = 1000000)]
    pub max_files: usize,

    /// Maximum number of failures before stopping
    #[arg(long, default_value_t = 50)]
    pub max_failures: usize,

    /// Copy failed files to output directory
    #[arg(long)]
    pub copy_failed_files: bool,

    /// Parse files in parallel
    #[arg(short, long)]
    pub parallel: bool,
    
    /// Whether to use the advanced parser (true) or simple parser (false)
    /// This is deprecated in favor of --parser-type
    #[arg(long, default_value_t = true, hide = true)]
    pub use_advanced: bool,
    
    /// Which parser to use (simple or advanced)
    #[arg(long, default_value = "advanced")]
    pub parser_type: ParserType,
} 