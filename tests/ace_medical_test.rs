use gamedata_scanner::{Scanner, ScannerConfig};
use std::collections::HashSet;
use std::path::PathBuf;

#[test]
fn test_ace_medical_classes() {
    // Initialize test environment
    let _ = env_logger::builder().is_test(true).try_init();

    // Get path to test fixture
    let mut fixture_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixture_path.push("tests");
    fixture_path.push("fixtures");
    
    // Create scanner with test configuration
    let mut config = ScannerConfig::default();
    config.show_progress = false;  // Disable progress bar in tests
    let scanner = Scanner::new(config);
    
    // Scan the fixtures directory
    let result = scanner.scan_directory(&fixture_path).unwrap();
    
    // Verify basic scan results
    assert_eq!(result.total_files, 1, "Expected exactly one file to be processed");
    assert_eq!(result.successful_files, 1, "Expected file to be processed successfully");
    assert_eq!(result.failed_files, 0, "Expected no failures");
    
    // Get all classes from the scan result
    let mut found_classes = HashSet::new();
    for scan_result in result.results.values() {
        for class in &scan_result.classes {
            found_classes.insert(class.name.clone());
        }
    }
    
    // Define expected classes from ace_cfg.hpp
    let expected_classes = vec![
        "CfgWeapons",
        "ACE_fieldDressing",
        "ACE_packingBandage",
        "ACE_elasticBandage",
        "ACE_tourniquet",
        "ACE_splint",
        "ACE_morphine",
        "ACE_adenosine",
        "ACE_atropine",
        "ACE_epinephrine",
        "ACE_plasmaIV",
        "ACE_plasmaIV_500",
        "ACE_plasmaIV_250",
        "ACE_bloodIV",
        "ACE_bloodIV_500",
        "ACE_bloodIV_250",
        "ACE_salineIV",
        "ACE_salineIV_500",
        "ACE_salineIV_250",
        "ACE_quikclot",
        "ACE_personalAidKit",
        "ACE_surgicalKit",
        "ACE_suture",
        "ACE_bodyBag",
        "ACE_bodyBag_blue",
        "ACE_bodyBag_white",
    ];
    
    // Verify all expected classes were found
    for class_name in &expected_classes {
        assert!(
            found_classes.contains(*class_name),
            "Missing expected class: {}",
            class_name
        );
    }
    
    // Verify class inheritance
    let mut found_class_with_inheritance = false;
    for scan_result in result.results.values() {
        for class in &scan_result.classes {
            match class.name.as_str() {
                // Check some specific inheritance relationships
                "FirstAidKit" => {
                    assert_eq!(class.parent.as_deref(), Some("ItemCore"));
                    found_class_with_inheritance = true;
                }
                "ACE_fieldDressing" | "ACE_packingBandage" | "ACE_elasticBandage" => {
                    assert_eq!(class.parent.as_deref(), Some("ACE_ItemCore"));
                    found_class_with_inheritance = true;
                }
                "ACE_bodyBag_blue" | "ACE_bodyBag_white" => {
                    assert_eq!(class.parent.as_deref(), Some("ACE_bodyBag"));
                    found_class_with_inheritance = true;
                }
                _ => {}
            }
        }
    }
    assert!(found_class_with_inheritance, "No classes with inheritance relationships were found");
    
    // Verify class properties
    let mut found_class_with_properties = false;
    for scan_result in result.results.values() {
        for class in &scan_result.classes {
            if class.name == "ACE_fieldDressing" {
                // Check for specific properties we expect to find
                let properties: HashSet<_> = class.properties.iter()
                    .map(|p| p.name.as_str())
                    .collect();
                
                assert!(properties.contains("scope"));
                assert!(properties.contains("author"));
                assert!(properties.contains("displayName"));
                assert!(properties.contains("picture"));
                assert!(properties.contains("descriptionShort"));
                assert!(properties.contains("ACE_isMedicalItem"));
                
                found_class_with_properties = true;
                break;
            }
        }
    }
    assert!(found_class_with_properties, "Class properties were not found as expected");
    
    // Verify nested classes (ItemInfo)
    let mut found_nested_class = false;
    for scan_result in result.results.values() {
        for class in &scan_result.classes {
            if class.name == "ItemInfo" {
                found_nested_class = true;
                break;
            }
        }
    }
    assert!(found_nested_class, "Nested ItemInfo classes were not found");
} 