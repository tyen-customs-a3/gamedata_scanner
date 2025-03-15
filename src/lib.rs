mod config;
mod models;
mod error;
mod scanner;
mod utils;

// Re-export everything from the modules
pub use config::{GameDataScannerConfig, GameDataScannerConfigBuilder};
pub use models::{FileResult, ScanResult, ClassMap};
pub use error::{ScanError, ScanResult as ResultType};
pub use scanner::{scan_directory, filter_classes};
pub use utils::{get_derived_classes, get_classes_with_property, get_classes_with_property_value};

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use parser_code::{CodeClass, CodeProperty, CodeValue};

    fn create_test_class_map() -> ClassMap {
        let mut class_map = HashMap::new();
        
        // Base class
        let base_vehicle = CodeClass {
            name: "BaseVehicle".to_string(),
            parent: None,
            properties: vec![
                CodeProperty {
                    name: "scope".to_string(),
                    value: CodeValue::Number(2),
                },
                CodeProperty {
                    name: "displayName".to_string(),
                    value: CodeValue::String("Base Vehicle".to_string()),
                }
            ],
        };
        
        // Derived classes
        let car = CodeClass {
            name: "Car".to_string(),
            parent: Some("BaseVehicle".to_string()),
            properties: vec![
                CodeProperty {
                    name: "maxSpeed".to_string(),
                    value: CodeValue::Number(100),
                },
                CodeProperty {
                    name: "displayName".to_string(),
                    value: CodeValue::String("Car".to_string()),
                }
            ],
        };
        
        let truck = CodeClass {
            name: "Truck".to_string(),
            parent: Some("BaseVehicle".to_string()),
            properties: vec![
                CodeProperty {
                    name: "maxSpeed".to_string(),
                    value: CodeValue::Number(80),
                },
                CodeProperty {
                    name: "cargoCapacity".to_string(),
                    value: CodeValue::Number(1000),
                }
            ],
        };
        
        class_map.insert("BaseVehicle".to_string(), vec![base_vehicle]);
        class_map.insert("Car".to_string(), vec![car]);
        class_map.insert("Truck".to_string(), vec![truck]);
        
        class_map
    }

    #[test]
    fn test_get_derived_classes() {
        let class_map = create_test_class_map();
        let scan_result = ScanResult::new("/test");
        let mut scan_result = ScanResult {
            class_map,
            ..scan_result
        };
        
        // Update counts
        scan_result.files_scanned = 1;
        scan_result.classes_found = 3;
        
        let derived = get_derived_classes(&scan_result, "BaseVehicle");
        assert_eq!(derived.len(), 2);
        assert!(derived.contains(&"Car".to_string()));
        assert!(derived.contains(&"Truck".to_string()));
        
        // Test with non-existent base class
        let derived = get_derived_classes(&scan_result, "NonExistentClass");
        assert!(derived.is_empty());
    }

    #[test]
    fn test_get_classes_with_property() {
        let class_map = create_test_class_map();
        let scan_result = ScanResult::new("/test");
        let mut scan_result = ScanResult {
            class_map,
            ..scan_result
        };
        
        // Update counts
        scan_result.files_scanned = 1;
        scan_result.classes_found = 3;
        
        // Test finding classes with maxSpeed property
        let with_max_speed = get_classes_with_property(&scan_result, "maxSpeed");
        assert_eq!(with_max_speed.len(), 2);
        assert!(with_max_speed.contains(&"Car".to_string()));
        assert!(with_max_speed.contains(&"Truck".to_string()));
        
        // Test finding classes with displayName property
        let with_display_name = get_classes_with_property(&scan_result, "displayName");
        assert_eq!(with_display_name.len(), 2);
        assert!(with_display_name.contains(&"BaseVehicle".to_string()));
        assert!(with_display_name.contains(&"Car".to_string()));
        
        // Test with non-existent property
        let with_nonexistent = get_classes_with_property(&scan_result, "nonexistentProperty");
        assert!(with_nonexistent.is_empty());
    }
    
    #[test]
    fn test_get_classes_with_property_value() {
        let class_map = create_test_class_map();
        let scan_result = ScanResult::new("/test");
        let mut scan_result = ScanResult {
            class_map,
            ..scan_result
        };
        
        // Update counts
        scan_result.files_scanned = 1;
        scan_result.classes_found = 3;
        
        // Find classes with maxSpeed >= 100
        let fast_vehicles = get_classes_with_property_value(&scan_result, "maxSpeed", |value| {
            if let CodeValue::Number(n) = value {
                *n >= 100
            } else {
                false
            }
        });
        
        assert_eq!(fast_vehicles.len(), 1);
        assert!(fast_vehicles.contains(&"Car".to_string()));
    }
}
