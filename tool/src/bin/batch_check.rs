use clap::Parser;
use batch_scanner::config::Args;

fn main() -> std::io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();
    
    // Run the batch parser
    batch_scanner::run(args).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
} 