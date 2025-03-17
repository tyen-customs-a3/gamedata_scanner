use std::path::PathBuf;
use gamedata_scanner::{Scanner, ScannerConfig};
use env_logger::Env;
use log::{info, error};
use std::time::Instant;

fn main() {
    // Initialize logging
    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_millis()
        .init();

    // Parse command line arguments
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <directory_to_scan>", args[0]);
        std::process::exit(1);
    }

    let input_dir = PathBuf::from(&args[1]);
    
    // Create scanner with custom configuration
    let mut config = ScannerConfig::default();
    config.show_progress = true;
    
    let scanner = Scanner::new(config);
    
    info!("Starting scan of directory: {}", input_dir.display());
    let start_time = Instant::now();
    
    // Perform the scan
    match scanner.scan_directory(&input_dir) {
        Ok(result) => {
            let elapsed = start_time.elapsed();
            
            info!("Scan completed in {:.2} seconds", elapsed.as_secs_f64());
            info!("Total files processed: {}", result.total_files);
            info!("Successfully processed: {}", result.successful_files);
            info!("Failed to process: {}", result.failed_files);
            
            // Print summary of found classes
            let total_classes: usize = result.results.values()
                .map(|r| r.classes.len())
                .sum();
            info!("Total classes found: {}", total_classes);
            
            // Print details about failed files
            if !result.errors.is_empty() {
                info!("\nFiles that failed to process:");
                for (path, error) in result.errors {
                    error!("  {}: {}", path.display(), error);
                }
            }
            
            // Print some statistics about found classes
            if !result.results.is_empty() {
                info!("\nClass statistics:");
                let mut class_counts = std::collections::HashMap::new();
                
                for result in result.results.values() {
                    for class in &result.classes {
                        if let Some(parent) = &class.parent {
                            *class_counts.entry(parent.clone()).or_insert(0) += 1;
                        }
                    }
                }
                
                // Print top 10 base classes by number of derived classes
                let mut counts: Vec<_> = class_counts.into_iter().collect();
                counts.sort_by(|a, b| b.1.cmp(&a.1));
                
                info!("Top 10 base classes by number of derived classes:");
                for (base_class, count) in counts.iter().take(10) {
                    info!("  {}: {} derived classes", base_class, count);
                }
            }
        },
        Err(e) => {
            error!("Failed to scan directory: {}", e);
            std::process::exit(1);
        }
    }
} 