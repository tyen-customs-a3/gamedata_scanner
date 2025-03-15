use crate::models::ScanResult;

/// Get all classes that inherit from a specific base class
pub fn get_derived_classes(scan_result: &ScanResult, base_class: &str) -> Vec<String> {
    let mut derived_classes = Vec::new();
    
    for (class_name, class_instances) in &scan_result.class_map {
        for class in class_instances {
            if let Some(parent) = &class.parent {
                if parent == base_class {
                    derived_classes.push(class_name.clone());
                }
            }
        }
    }
    
    derived_classes
}

/// Get all classes with a specific property
pub fn get_classes_with_property(scan_result: &ScanResult, property_name: &str) -> Vec<String> {
    let mut matching_classes = Vec::new();
    
    for (class_name, class_instances) in &scan_result.class_map {
        for class in class_instances {
            if class.properties.iter().any(|prop| prop.name == property_name) {
                matching_classes.push(class_name.clone());
                break;
            }
        }
    }
    
    matching_classes
} 