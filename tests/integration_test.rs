use std::collections::HashMap;
use gamedata_scanner::{
    scan_directory,
    GameDataScannerConfig,
    get_derived_classes,
};
use std::path::PathBuf;

// Test data structure for class inheritance
#[derive(Debug)]
struct ExpectedClass {
    name: &'static str,
    parent: Option<&'static str>,
    properties: Vec<(&'static str, &'static str)>, // (name, expected_value)
}

// Helper function to get the fixtures directory
fn get_fixtures_dir() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir).join("tests").join("fixtures")
}

// Helper function to print class information for debugging
fn debug_print_class_info(class_map: &HashMap<String, Vec<parser_code::CodeClass>>) {
    println!("\n=== Debug Class Information ===");
    for (class_name, instances) in class_map {
        for class in instances {
            println!("\nClass: {}", class_name);
            println!("Parent: {:?}", class.parent);
            println!("Properties:");
            for prop in &class.properties {
                println!("  {} = {:?}", prop.name, prop.value);
            }
        }
    }
    println!("============================\n");
}

// Helper function to normalize property values for comparison
fn normalize_value(value: &str) -> String {
    // Remove Number(), String(), etc. wrappers and quotes
    let value = value.replace("Number(", "").replace("String(", "").replace(")", "");
    value.trim_matches('"').to_string()
}

// Expected classes for headgear pumpkin mod
fn get_headgear_pumpkin_classes() -> Vec<ExpectedClass> {
    vec![
        ExpectedClass {
            name: "TC_Helmet_HalloweenPumpkin",
            parent: Some("HelmetBase"),
            properties: vec![
                ("scope", "2"),
                ("displayName", "Pumpkin (Halloween)"),
                ("author", "Tyen"),
                ("descriptionShort", "Spooky Pumpkin Hat"),
            ],
        },
        ExpectedClass {
            name: "TC_NVG_HalloweenPumpkin",
            parent: Some("NVGoggles"),
            properties: vec![
                ("scope", "2"),
                ("displayName", "Pumpkin (Halloween)"),
                ("author", "Tyen"),
                ("descriptionShort", "Pumpkin, Halloween"),
            ],
        },
        ExpectedClass {
            name: "TC_G_HalloweenPumpkin",
            parent: Some("None"),
            properties: vec![
                ("scope", "2"),
                ("displayName", "Pumpkin (Halloween)"),
                ("author", "Tyen"),
                ("descriptionShort", "Pumpkin, Halloween"),
                ("mass", "5"),
            ],
        },
    ]
}

// Expected classes for ACE medical mod
fn get_ace_medical_classes() -> Vec<ExpectedClass> {
    vec![
        ExpectedClass {
            name: "FirstAidKit",
            parent: Some("ItemCore"),
            properties: vec![
                ("type", "0"),
                ("ACE_isMedicalItem", "1"),
            ],
        },
        ExpectedClass {
            name: "ACE_fieldDressing",
            parent: Some("ACE_ItemCore"),
            properties: vec![
                ("scope", "2"),
                ("ACE_isMedicalItem", "1"),
                ("displayName", "CSTRING(Bandage_Basic_Display)"),
                ("descriptionShort", "CSTRING(Bandage_Basic_Desc_Short)"),
            ],
        },
        ExpectedClass {
            name: "ACE_morphine",
            parent: Some("ACE_ItemCore"),
            properties: vec![
                ("scope", "2"),
                ("ACE_isMedicalItem", "1"),
                ("displayName", "CSTRING(Morphine_Display)"),
                ("descriptionShort", "CSTRING(Morphine_Desc_Short)"),
            ],
        },
        ExpectedClass {
            name: "ACE_bloodIV",
            parent: Some("ACE_ItemCore"),
            properties: vec![
                ("scope", "2"),
                ("ACE_isMedicalItem", "1"),
                ("displayName", "CSTRING(Blood_IV)"),
                ("descriptionShort", "CSTRING(Blood_IV_Desc_Short)"),
            ],
        },
    ]
}

// Test helper to verify class properties
fn verify_class_properties(class_map: &HashMap<String, Vec<parser_code::CodeClass>>, expected: &ExpectedClass) {
    if let Some(classes) = class_map.get(expected.name) {
        let class = &classes[0];
        
        println!("\nVerifying class: {}", expected.name);
        println!("Found parent: {:?}, Expected parent: {:?}", class.parent, expected.parent);
        
        // Verify parent
        if let Some(expected_parent) = expected.parent {
            assert_eq!(class.parent.as_deref(), Some(expected_parent),
                "Class {} should have parent {}", expected.name, expected_parent);
        }
        
        // Verify properties
        for (prop_name, expected_value) in &expected.properties {
            if let Some(prop) = class.properties.iter().find(|p| p.name == *prop_name) {
                let actual_value = normalize_value(&format!("{:?}", prop.value));
                let expected_value = normalize_value(expected_value);
                println!("Property {} = {:?}, Expected = {}", prop_name, prop.value, expected_value);
                assert_eq!(actual_value, expected_value,
                    "Property {} in class {} should have value {}", prop_name, expected.name, expected_value);
            } else {
                panic!("Property {} not found in class {}", prop_name, expected.name);
            }
        }
    } else {
        panic!("Class {} not found in class map", expected.name);
    }
}

#[tokio::test]
async fn test_scan_ace_medical() {
    let fixtures_dir = get_fixtures_dir();
    let mod_dir = fixtures_dir.join("ace_medical_treatment");
    
    let config = GameDataScannerConfig::builder()
        .with_extensions(vec!["cpp".to_string(), "hpp".to_string()])
        .build();
    
    let result = scan_directory(&mod_dir, &config).await.unwrap();
    
    println!("\n=== ACE Medical Test Results ===");
    println!("Files scanned: {}", result.files_scanned);
    println!("Classes found: {}", result.classes_found);
    println!("Files with errors: {}", result.files_with_errors);
    
    debug_print_class_info(&result.class_map);
    
    // Basic scan results
    assert!(result.files_scanned > 0, "No files were scanned");
    assert!(result.classes_found > 0, "No classes were found");
    assert_eq!(result.files_with_errors, 0, "Some files had parsing errors");
    
    // Verify expected classes and their properties
    let expected_classes = get_ace_medical_classes();
    for expected_class in expected_classes {
        verify_class_properties(&result.class_map, &expected_class);
    }
    
    // Verify inheritance relationships
    let ace_itemcore_derived = get_derived_classes(&result, "ACE_ItemCore");
    println!("\nClasses derived from ACE_ItemCore: {:?}", ace_itemcore_derived);
    assert!(ace_itemcore_derived.contains(&"ACE_fieldDressing".to_string()));
    assert!(ace_itemcore_derived.contains(&"ACE_morphine".to_string()));
    assert!(ace_itemcore_derived.contains(&"ACE_bloodIV".to_string()));
}

#[tokio::test]
async fn test_scan_headgear_pumpkin_mod() {
    let fixtures_dir = get_fixtures_dir();
    let mod_dir = fixtures_dir.join("headgear_pumpkin");
    
    let config = GameDataScannerConfig::builder()
        .with_extensions(vec!["cpp".to_string()])
        .build();
    
    let result = scan_directory(&mod_dir, &config).await.unwrap();
    
    println!("\n=== Headgear Pumpkin Test Results ===");
    println!("Files scanned: {}", result.files_scanned);
    println!("Classes found: {}", result.classes_found);
    println!("Files with errors: {}", result.files_with_errors);
    
    debug_print_class_info(&result.class_map);
    
    // Basic scan results
    assert!(result.files_scanned > 0, "No files were scanned");
    assert!(result.classes_found > 0, "No classes were found");
    assert_eq!(result.files_with_errors, 0, "Some files had parsing errors");
    
    // Verify expected classes and their properties
    let expected_classes = get_headgear_pumpkin_classes();
    for expected_class in expected_classes {
        verify_class_properties(&result.class_map, &expected_class);
    }
    
    // Verify inheritance relationships
    let helmet_derived = get_derived_classes(&result, "HelmetBase");
    println!("\nClasses derived from HelmetBase: {:?}", helmet_derived);
    assert!(helmet_derived.contains(&"TC_Helmet_HalloweenPumpkin".to_string()));
    
    let nvg_derived = get_derived_classes(&result, "NVGoggles");
    println!("\nClasses derived from NVGoggles: {:?}", nvg_derived);
    assert!(nvg_derived.contains(&"TC_NVG_HalloweenPumpkin".to_string()));
}

#[tokio::test]
async fn test_scan_multiple_mods() {
    let fixtures_dir = get_fixtures_dir();
    
    let config = GameDataScannerConfig::builder()
        .with_extensions(vec!["cpp".to_string()])
        .build();
    
    let result = scan_directory(&fixtures_dir, &config).await.unwrap();
    
    println!("\n=== Multiple Mods Test Results ===");
    println!("Files scanned: {}", result.files_scanned);
    println!("Classes found: {}", result.classes_found);
    println!("Files with errors: {}", result.files_with_errors);
    
    debug_print_class_info(&result.class_map);
    
    // Basic scan results
    assert!(result.files_scanned > 0, "No files were scanned");
    assert!(result.classes_found > 0, "No classes were found");
    
    // Verify classes from both mods
    let headgear_classes = get_headgear_pumpkin_classes();
    let medical_classes = get_ace_medical_classes();
    
    // Verify a sample of classes from each mod
    verify_class_properties(&result.class_map, &headgear_classes[0]); // TC_Helmet_HalloweenPumpkin
    verify_class_properties(&result.class_map, &medical_classes[0]); // FirstAidKit
    
    // Verify cross-mod inheritance
    let item_core_derived = get_derived_classes(&result, "ItemCore");
    println!("\nClasses derived from ItemCore: {:?}", item_core_derived);
    assert!(item_core_derived.contains(&"FirstAidKit".to_string()));
    assert!(item_core_derived.contains(&"TC_Helmet_HalloweenPumpkin".to_string()));
}

#[tokio::test]
async fn test_scan_with_file_limit() {
    let fixtures_dir = get_fixtures_dir();
    
    let config = GameDataScannerConfig::builder()
        .with_extensions(vec!["cpp".to_string()])
        .with_max_files(Some(1))
        .build();
    
    let result = scan_directory(&fixtures_dir, &config).await.unwrap();
    
    println!("\n=== File Limit Test Results ===");
    println!("Files scanned: {}", result.files_scanned);
    println!("Classes found: {}", result.classes_found);
    println!("Files with errors: {}", result.files_with_errors);
    
    debug_print_class_info(&result.class_map);
    
    // Should only scan one file
    assert_eq!(result.files_scanned, 1, "Should only scan one file");
} 