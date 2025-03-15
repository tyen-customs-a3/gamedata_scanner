use crate::models::ScanResult;

/// Get all classes that inherit from a specific base class
///
/// # Arguments
///
/// * `scan_result` - The scan results to search in
/// * `base_class` - The name of the base class to search for
///
/// # Returns
///
/// A vector of class names that inherit from the specified base class
pub fn get_derived_classes(scan_result: &ScanResult, base_class: &str) -> Vec<String> {
    scan_result.class_map
        .iter()
        .filter_map(|(class_name, instances)| {
            if instances.iter().any(|class| class.parent.as_deref() == Some(base_class)) {
                Some(class_name.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Get all classes with a specific property
///
/// # Arguments
///
/// * `scan_result` - The scan results to search in
/// * `property_name` - The name of the property to search for
///
/// # Returns
///
/// A vector of class names that have the specified property
pub fn get_classes_with_property(scan_result: &ScanResult, property_name: &str) -> Vec<String> {
    scan_result.class_map
        .iter()
        .filter_map(|(class_name, instances)| {
            if instances.iter().any(|class| 
                class.properties.iter().any(|prop| prop.name == property_name)) {
                Some(class_name.clone())
            } else {
                None
            }
        })
        .collect()
}

/// Get all classes that have a property with a specific value
///
/// # Arguments
///
/// * `scan_result` - The scan results to search in
/// * `property_name` - The name of the property to search for
/// * `value_predicate` - A function that returns true if the property value matches
///
/// # Returns
///
/// A vector of class names that have the specified property with a value that matches the predicate
pub fn get_classes_with_property_value<F>(
    scan_result: &ScanResult, 
    property_name: &str,
    value_predicate: F
) -> Vec<String> 
where 
    F: Fn(&parser_code::CodeValue) -> bool
{
    scan_result.class_map
        .iter()
        .filter_map(|(class_name, instances)| {
            if instances.iter().any(|class| 
                class.properties.iter().any(|prop| 
                    prop.name == property_name && value_predicate(&prop.value))) {
                Some(class_name.clone())
            } else {
                None
            }
        })
        .collect()
} 