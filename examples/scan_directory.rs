use std::path::PathBuf;
use std::env;
use gamedata_scanner::{
    scan_directory,
    GameDataScannerConfig,
    get_derived_classes,
    get_classes_with_property,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Get directory to scan from command line arguments or use current directory
    let args: Vec<String> = env::args().collect();
    let directory = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };
    
    println!("Scanning directory: {}", directory.display());
    
    // Create scanner configuration
    let mut config = GameDataScannerConfig::default();
    config.max_threads = num_cpus::get();
    config.follow_symlinks = false;
    
    // Scan the directory
    let result = scan_directory(&directory, &config).await
        .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
    
    println!("\n=== Scan Results ===");
    println!("Files scanned: {}", result.files_scanned);
    println!("Classes found: {}", result.classes_found);
    println!("Files with errors: {}", result.files_with_errors);
    println!("Total scan time: {}ms", result.total_scan_time_ms);
    
    // Print some example class information
    println!("\n=== Class Information ===");
    
    // Get the top 10 classes by number of properties
    let mut classes_by_properties: Vec<_> = result.class_map.iter()
        .map(|(name, instances)| {
            let total_props = instances.iter()
                .map(|class| class.properties.len())
                .sum::<usize>();
            (name, total_props)
        })
        .collect();
    
    classes_by_properties.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\nTop 10 classes by number of properties:");
    for (i, (class_name, prop_count)) in classes_by_properties.iter().take(10).enumerate() {
        println!("{}. {} - {} properties", i + 1, class_name, prop_count);
    }
    
    // Find classes that inherit from a specific base class
    if !result.class_map.is_empty() {
        let base_class = classes_by_properties.first().unwrap().0;
        let derived = get_derived_classes(&result, base_class);
        
        println!("\nClasses that inherit from '{}':", base_class);
        for (i, class) in derived.iter().enumerate() {
            println!("{}. {}", i + 1, class);
            if i >= 9 {
                println!("... and {} more", derived.len() - 10);
                break;
            }
        }
        
        // Find classes with a specific property
        let property_name = "displayName";
        let with_property = get_classes_with_property(&result, property_name);
        
        println!("\nClasses with '{}' property:", property_name);
        for (i, class) in with_property.iter().enumerate() {
            println!("{}. {}", i + 1, class);
            if i >= 9 {
                println!("... and {} more", with_property.len() - 10);
                break;
            }
        }
    }
    
    // Print file statistics
    println!("\n=== File Statistics ===");
    let mut files_by_classes: Vec<_> = result.file_results.iter()
        .map(|file| (file.file_path.display().to_string(), file.classes.len()))
        .collect();
    
    files_by_classes.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\nTop 10 files by number of classes:");
    for (i, (file_path, class_count)) in files_by_classes.iter().take(10).enumerate() {
        println!("{}. {} - {} classes", i + 1, file_path, class_count);
    }
    
    // Print any errors
    if result.files_with_errors > 0 {
        println!("\n=== Files with Errors ===");
        for file_result in &result.file_results {
            if !file_result.errors.is_empty() {
                println!("File: {}", file_result.file_path.display());
                for (i, error) in file_result.errors.iter().enumerate() {
                    println!("  {}. {}", i + 1, error);
                }
            }
        }
    }
    
    Ok(())
} 