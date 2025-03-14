use std::path::Path;
use std::fs::{self, File};
use std::io::Write;
use tempfile::TempDir;
use gamedata_scanner::{
    scan_directory,
    GameDataScannerConfig,
    get_derived_classes,
    get_classes_with_property,
};

#[tokio::test]
async fn test_scan_directory_with_nested_classes() {
    // Create a temporary directory structure
    let temp_dir = TempDir::new().unwrap();
    let nested_dir = temp_dir.path().join("nested");
    fs::create_dir(&nested_dir).unwrap();
    
    // Create test files with various class structures
    let base_content = r#"
        class BaseVehicle {
            scope = 2;
            displayName = "Base Vehicle";
            model = "\A3\data_f\default.p3d";
        };
        
        class BaseCar: BaseVehicle {
            armor = 100;
            fuelCapacity = 100;
        };
    "#;
    
    let derived_content = r#"
        class SportsCar: BaseCar {
            displayName = "Sports Car";
            maxSpeed = 200;
            armor = 50;
            
            class Turret {
                weapons[] = {"HMG_127"};
                magazines[] = {"100Rnd_127x99_mag", "100Rnd_127x99_mag"};
            };
        };
        
        class Truck: BaseCar {
            displayName = "Truck";
            maxSpeed = 100;
            armor = 150;
            fuelCapacity = 200;
        };
    "#;
    
    let nested_content = r#"
        class CfgWeapons {
            class Rifle_Base_F {
                scope = 0;
                magazines[] = {};
            };
            
            class arifle_MX_Base_F: Rifle_Base_F {
                scope = 1;
                magazines[] = {"30Rnd_65x39_caseless_mag"};
            };
            
            class arifle_MX_F: arifle_MX_Base_F {
                scope = 2;
                displayName = "MX 6.5 mm";
                picture = "\A3\weapons_F\Rifles\MX\data\UI\gear_mx_rifle_X_CA.paa";
            };
        };
    "#;
    
    // Write the test files
    write_file(temp_dir.path().join("base.hpp"), base_content);
    write_file(temp_dir.path().join("derived.cpp"), derived_content);
    write_file(nested_dir.join("weapons.hpp"), nested_content);
    
    // Create scanner configuration
    let config = GameDataScannerConfig::default();
    
    // Scan the directory
    let result = scan_directory(temp_dir.path(), &config).await.unwrap();
    
    // Verify basic results
    assert_eq!(result.files_scanned, 3);
    assert!(result.classes_found >= 7, "Expected at least 7 classes, found {}", result.classes_found);
    
    // Check class hierarchy
    let base_car_derived = get_derived_classes(&result, "BaseCar");
    assert_eq!(base_car_derived.len(), 2);
    assert!(base_car_derived.contains(&"SportsCar".to_string()));
    assert!(base_car_derived.contains(&"Truck".to_string()));
    
    let rifle_base_derived = get_derived_classes(&result, "Rifle_Base_F");
    assert_eq!(rifle_base_derived.len(), 1);
    assert!(rifle_base_derived.contains(&"arifle_MX_Base_F".to_string()));
    
    // Check properties
    let with_display_name = get_classes_with_property(&result, "displayName");
    assert!(with_display_name.len() >= 4);
    assert!(with_display_name.contains(&"BaseVehicle".to_string()));
    assert!(with_display_name.contains(&"SportsCar".to_string()));
    assert!(with_display_name.contains(&"Truck".to_string()));
    assert!(with_display_name.contains(&"arifle_MX_F".to_string()));
    
    // Check nested classes
    assert!(result.class_map.contains_key("CfgWeapons"));
    
    // Check property values for a specific class
    let sports_car = &result.class_map["SportsCar"][0];
    let max_speed_prop = sports_car.properties.iter()
        .find(|p| p.name == "maxSpeed")
        .expect("maxSpeed property not found");
    
    match &max_speed_prop.value {
        parser_code::CodeValue::Number(value) => assert_eq!(*value, 200),
        _ => panic!("maxSpeed should be a number"),
    }
}

fn write_file(path: impl AsRef<Path>, content: &str) {
    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
} 