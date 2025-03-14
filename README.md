# Game Data Scanner

A Rust library for scanning Arma 3 game data files (.cpp and .hpp) and extracting class definitions.

## Features

- Recursively scan directories for .cpp and .hpp files
- Parse class definitions using the parser_code subcrate
- Extract class hierarchies and properties
- Parallel processing for improved performance
- Detailed reporting of scan results

## Usage

### Basic Usage

```rust
use std::path::Path;
use gamedata_scanner::{scan_directory, GameDataScannerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();
    
    // Create scanner configuration
    let config = GameDataScannerConfig::default();
    
    // Scan a directory
    let result = scan_directory(Path::new("path/to/game/data"), &config).await?;
    
    // Print results
    println!("Files scanned: {}", result.files_scanned);
    println!("Classes found: {}", result.classes_found);
    
    Ok(())
}
```

### Configuration Options

```rust
let mut config = GameDataScannerConfig::default();

// Specify file extensions to scan
config.file_extensions = vec!["cpp".to_string(), "hpp".to_string()];

// Set maximum number of threads for parallel processing
config.max_threads = 4;

// Whether to follow symbolic links
config.follow_symlinks = false;

// Whether to use caching (not yet implemented)
config.use_cache = true;
```

### Working with Scan Results

```rust
// Get all classes that inherit from a specific base class
let derived_classes = get_derived_classes(&result, "BaseMan");

// Get all classes with a specific property
let classes_with_property = get_classes_with_property(&result, "displayName");

// Access the class map directly
for (class_name, class_instances) in &result.class_map {
    println!("Class: {}", class_name);
    
    for class in class_instances {
        if let Some(parent) = &class.parent {
            println!("  Parent: {}", parent);
        }
        
        println!("  Properties: {}", class.properties.len());
    }
}
```

## Examples

See the `examples` directory for complete examples:

- `scan_directory.rs` - Demonstrates how to scan a directory and analyze the results

Run an example:

```bash
cargo run --example scan_directory -- /path/to/game/data
```

## Integration with Other Tools

This library is designed to work with other components of the Arma 3 Tool:

- Use with `mission_scanner` to analyze mission dependencies
- Combine with extraction tools to process PBO files
- Generate reports on class usage and dependencies

## License

This project is licensed under the MIT License - see the LICENSE file for details. 