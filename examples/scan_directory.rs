use std::path::PathBuf;
use std::env;
use std::fs::File;
use std::io::Write;
use gamedata_scanner::{
    scan_directory,
    GameDataScannerConfig,
    get_derived_classes,
    get_classes_with_property,
};
use serde::{Serialize, Deserialize};
use chrono;
use serde_json;

#[derive(Serialize, Deserialize)]
struct ScanReport {
    timestamp: String,
    files_scanned: usize,
    classes_found: usize,
    files_with_errors: usize,
    scan_time_ms: u64,
    top_classes: Vec<(String, usize)>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    
    // Extract flags and options from arguments
    let mut dir_path = None;
    let mut max_files = None;
    let mut report_path = None;
    let mut diagnostic_path = None;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--max-files" => {
                if i + 1 < args.len() {
                    match args[i+1].parse::<usize>() {
                        Ok(num) => {
                            max_files = Some(num);
                            i += 2;
                        },
                        Err(_) => {
                            eprintln!("Error: --max-files requires a number");
                            return Ok(());
                        }
                    }
                } else {
                    eprintln!("Error: --max-files requires a value");
                    return Ok(());
                }
            },
            "--report-path" => {
                if i + 1 < args.len() {
                    report_path = Some(PathBuf::from(&args[i+1]));
                    i += 2;
                } else {
                    eprintln!("Error: --report-path requires a value");
                    return Ok(());
                }
            },
            "--diagnostic-path" => {
                if i + 1 < args.len() {
                    diagnostic_path = Some(PathBuf::from(&args[i+1]));
                    i += 2;
                } else {
                    eprintln!("Error: --diagnostic-path requires a value");
                    return Ok(());
                }
            },
            arg => {
                // First non-flag argument is the directory
                if !arg.starts_with("--") && dir_path.is_none() {
                    dir_path = Some(PathBuf::from(arg));
                }
                i += 1;
            }
        }
    }
    
    // Get directory to scan from command line arguments or use current directory
    let directory = dir_path.unwrap_or_else(|| env::current_dir().unwrap());
    
    if let Some(limit) = max_files {
        println!("Scanning directory: {} (limited to {} files)", directory.display(), limit);
    } else {
        println!("Scanning directory: {}", directory.display());
    }
    
    // Create scanner configuration
    let mut config = GameDataScannerConfig::default();
    config.max_threads = num_cpus::get();
    config.max_files = max_files;
    
    // Scan the directory
    let result = scan_directory(&directory, &config).await
        .map_err(|e| Box::<dyn std::error::Error>::from(e))?;
    
    println!("\n=== Scan Results ===");
    println!("Files scanned: {}", result.files_scanned);
    if let Some(limit) = max_files {
        if result.files_scanned == limit {
            println!("(Scan limited to the first {} files)", limit);
        }
    }
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
            (name.clone(), total_props)
        })
        .collect();
    
    classes_by_properties.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\nTop 10 classes by number of properties:");
    for (i, (class_name, prop_count)) in classes_by_properties.iter().take(10).enumerate() {
        println!("{}. {} - {} properties", i + 1, class_name, prop_count);
    }
    
    // Save report if path was provided
    if let Some(path) = report_path {
        println!("\nSaving scan report to: {}", path.display());
        
        // Create report structure
        let report = ScanReport {
            timestamp: chrono::Local::now().to_rfc3339(),
            files_scanned: result.files_scanned,
            classes_found: result.classes_found,
            files_with_errors: result.files_with_errors,
            scan_time_ms: result.total_scan_time_ms,
            top_classes: classes_by_properties.iter().take(20).map(|(name, count)| (name.clone(), *count)).collect(),
        };
        
        // Write JSON report
        let json = serde_json::to_string_pretty(&report)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
    }
    
    // Save diagnostic information if path was provided
    if let Some(path) = diagnostic_path {
        println!("Saving diagnostic information to: {}", path.display());
        
        let mut file = File::create(path)?;
        
        writeln!(file, "=== Game Data Scanner Diagnostic Report ===")?;
        writeln!(file, "Generated: {}", chrono::Local::now().to_rfc3339())?;
        writeln!(file, "Directory: {}", directory.display())?;
        writeln!(file, "Files scanned: {}", result.files_scanned)?;
        writeln!(file, "Classes found: {}", result.classes_found)?;
        writeln!(file, "Files with errors: {}", result.files_with_errors)?;
        writeln!(file, "Scan time: {}ms", result.total_scan_time_ms)?;
        
        writeln!(file, "\n=== Files with Errors ===")?;
        for file_result in &result.file_results {
            if !file_result.errors.is_empty() {
                writeln!(file, "\nFile: {}", file_result.file_path.display())?;
                for (i, error) in file_result.errors.iter().enumerate() {
                    writeln!(file, "  {}. {}", i + 1, error)?;
                }
            }
        }
    }
    
    // Find classes that inherit from a specific base class
    if !result.class_map.is_empty() {
        let base_class_name = &classes_by_properties.first().unwrap().0;
        let derived = get_derived_classes(&result, base_class_name);
        
        println!("\nClasses that inherit from '{}':", base_class_name);
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
    
    Ok(())
} 