mod config;
mod models;
mod error;
mod scanner;
mod utils;

// Re-export everything from the modules
pub use config::GameDataScannerConfig;
pub use models::{FileResult, ScanResult};
pub use error::ScanError;
pub use scanner::scan_directory;
pub use utils::{get_derived_classes, get_classes_with_property};

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    
    fn create_test_file(dir: &Path, filename: &str, content: &str) -> std::path::PathBuf {
        let file_path = dir.join(filename);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }
    
    #[tokio::test]
    async fn test_scan_directory() {
        // Create a temporary directory with test files
        let temp_dir = TempDir::new().unwrap();
        
        // Create test files
        let content1 = r#"
            class BaseMan {
                displayName = "Base";
            };
            class Rifleman : BaseMan {
                displayName = "Rifleman";
            };
        "#;
        
        let content2 = r#"
            class Vehicle {
                displayName = "Vehicle";
            };
            class Car : Vehicle {
                displayName = "Car";
                maxSpeed = 100;
            };
        "#;
        
        create_test_file(temp_dir.path(), "test1.hpp", content1);
        create_test_file(temp_dir.path(), "test2.cpp", content2);
        
        // Scan the directory
        let config = GameDataScannerConfig::default();
        let result = scan_directory(temp_dir.path(), &config).await.unwrap();
        
        // Verify results
        assert_eq!(result.files_scanned, 2);
        assert_eq!(result.classes_found, 4);
        assert_eq!(result.class_map.len(), 4);
        
        // Check that we found all classes
        assert!(result.class_map.contains_key("BaseMan"));
        assert!(result.class_map.contains_key("Rifleman"));
        assert!(result.class_map.contains_key("Vehicle"));
        assert!(result.class_map.contains_key("Car"));
        
        // Check inheritance
        let derived = get_derived_classes(&result, "BaseMan");
        assert_eq!(derived.len(), 1);
        assert_eq!(derived[0], "Rifleman");
        
        // Check properties
        let with_max_speed = get_classes_with_property(&result, "maxSpeed");
        assert_eq!(with_max_speed.len(), 1);
        assert_eq!(with_max_speed[0], "Car");
    }
}
