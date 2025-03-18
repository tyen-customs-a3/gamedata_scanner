use clap::Parser;
use parser_code::batch_parser::{self, config::Args};

fn main() -> std::io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Run the batch parser
    batch_parser::run(args).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
} 