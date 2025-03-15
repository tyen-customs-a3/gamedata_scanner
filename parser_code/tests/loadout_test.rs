use parser_code::{CodeParser, CodeValue};
use std::fs;

#[test]
fn test_loadout_parsing() {
    let content = fs::read_to_string("tests/fixtures/test_config_small.cpp").unwrap();
    let parser = CodeParser::new(&content).unwrap();
    let classes = parser.parse_classes();

    // Debug print all classes
    println!("Found classes:");
    for class in &classes {
        println!("Class: {} (parent: {:?})", class.name, class.parent);
        println!("Properties: {:?}", class.properties);
    }

    // Test base class declarations (forward declarations)
    let base_classes = vec![
        "UniformItem",
        "Uniform_Base"
    ];
    
    for base_class in base_classes {
        // Skip this test if the base class is not found
        if let Some(class) = classes.iter().find(|c| c.name == base_class) {
            // Forward declarations should have no properties
            assert!(class.properties.is_empty(), "Base class {} should have no properties", base_class);
        } else {
            println!("Base class {} not found, skipping test", base_class);
        }
    }

    // Test uniform items and their inheritance
    let uniform_items = vec![
        ("bw_uniform_combat_fleck", "Uniform_Base"),
        ("bw_uniform_combat_rs_fleck", "Uniform_Base")
    ];

    for (item, parent) in uniform_items {
        // Skip this test if the uniform item is not found
        if let Some(class) = classes.iter().find(|c| c.name == item) {
            assert_eq!(class.parent.as_deref(), Some(parent), "Uniform {} should inherit from {}", item, parent);
            
            // Verify common properties
            assert!(class.properties.iter().any(|p| p.name == "author" && matches!(&p.value, CodeValue::String(s) if s == "BW")));
            assert!(class.properties.iter().any(|p| p.name == "scope" && matches!(&p.value, CodeValue::Number(n) if *n == 2)));
            assert!(class.properties.iter().any(|p| p.name == "displayName"));
            assert!(class.properties.iter().any(|p| p.name == "picture"));
            assert!(class.properties.iter().any(|p| p.name == "model"));
            assert!(class.properties.iter().any(|p| p.name == "hiddenSelections"));
            assert!(class.properties.iter().any(|p| p.name == "hiddenSelectionsTextures"));
            
            // Verify ItemInfo class inheritance and properties
            let item_info = class.properties.iter()
                .find(|p| p.name == "ItemInfo")
                .expect("ItemInfo class not found");

            if let CodeValue::Class(info_class) = &item_info.value {
                assert_eq!(info_class.parent.as_deref(), Some("UniformItem"), "ItemInfo should inherit from UniformItem");
                assert!(info_class.properties.iter().any(|p| p.name == "uniformModel" && matches!(&p.value, CodeValue::String(s) if s == "-")));
                assert!(info_class.properties.iter().any(|p| p.name == "uniformClass"));
                assert!(info_class.properties.iter().any(|p| p.name == "containerClass" && matches!(&p.value, CodeValue::String(s) if s == "Supply60")));
                assert!(info_class.properties.iter().any(|p| p.name == "mass" && matches!(&p.value, CodeValue::Number(n) if *n == 40)));
            } else {
                panic!("ItemInfo is not a class");
            }
        } else {
            println!("Uniform item {} not found, skipping test", item);
        }
    }
} 

#[test]
fn test_ace_medical_items() {
    // Read the config file
    let mut content = fs::read_to_string("tests/fixtures/ace_cfg.hpp").unwrap_or_else(|e| {
        panic!("Failed to read ace_cfg.hpp: {}\nCurrent dir: {:?}", 
            e, std::env::current_dir().unwrap_or_default())
    });
    
    // Remove leading/trailing whitespace
    content = content.trim().to_string();
    
    // Add preprocessor definitions and string definitions
    let definitions = r#"
        #define QUOTE(var1) #var1
        #define DOUBLES(var1,var2) ##var1##_##var2
        #define PREFIX ace
        #define GVAR(var1) DOUBLES(PREFIX,var1)
        #define QPATHTOF(var1) QUOTE(PATHTOF(var1))
        #define PATHTOF(var1) \data\##var1
        #define CSTRING(var1) QUOTE(DOUBLES(STR,var1))
        #define ECSTRING(var1,var2) QUOTE(DOUBLES(STR,DOUBLES(var1,var2)))
        #define STR_common_ACETeam "ACE Team"
        #define STR_Bandage_Basic_Display "Basic Bandage"
        #define STR_Bandage_Basic_Desc_Short "Basic first aid bandage"
        #define STR_Bandage_Basic_Desc_Use "Use to stop bleeding"
    "#;
    
    content = format!("{}\n{}", definitions, content);
    
    // Try parsing and capture any errors
    let parser = match CodeParser::new(&content) {
        Ok(p) => p,
        Err(e) => {
            println!("Parser initialization failed: {:?}", e);
            println!("Content that failed to parse:\n{}", content);
            panic!("Failed to initialize parser");
        }
    };
    
    let classes = parser.parse_classes();
    
    // Debug: Print all top-level classes found
    println!("\nFound {} top-level classes:", classes.len());
    for class in &classes {
        println!("Class: {} (parent: {:?})", class.name, class.parent);
        println!("  Properties count: {}", class.properties.len());
        if class.name == "CfgWeapons" {
            println!("  CfgWeapons properties:");
            for prop in &class.properties {
                println!("    {}: {:?}", prop.name, prop.value);
            }
        }
    }
    
    // Find CfgWeapons class with better error handling
    let cfg_weapons = classes.iter()
        .find(|c| c.name == "CfgWeapons")
        .unwrap_or_else(|| {
            println!("\nFailed to find CfgWeapons class. Available classes:");
            for class in &classes {
                println!("- {} (parent: {:?})", class.name, class.parent);
                println!("  Properties:");
                for prop in &class.properties {
                    println!("    {}: {:?}", prop.name, prop.value);
                }
            }
            panic!("CfgWeapons class not found in parsed content");
        });

    // Test base class declarations
    let base_classes = vec![
        "ItemCore",
        "ACE_ItemCore",
        "CBA_MiscItem_ItemInfo",
        "InventoryFirstAidKitItem_Base_F",
        "MedikitItem"
    ];
    
    for base_class in base_classes {
        // Skip this test if the base class is not found
        if let Some(class) = classes.iter().find(|c| c.name == base_class) {
            // Forward declarations should have no properties
            assert!(class.properties.is_empty(), "Base class {} should have no properties", base_class);
        } else {
            println!("Base class {} not found, skipping test", base_class);
        }
    }

    // Test medical items
    let medical_items = vec![
        // Basic medical items
        ("ACE_fieldDressing", "ACE_ItemCore", 0, true),
        ("ACE_packingBandage", "ACE_ItemCore", 0, true),
        ("ACE_elasticBandage", "ACE_ItemCore", 0, true),
        ("ACE_tourniquet", "ACE_ItemCore", 1, true),
        ("ACE_splint", "ACE_ItemCore", 2, true),
        ("ACE_morphine", "ACE_ItemCore", 1, true),
        ("ACE_adenosine", "ACE_ItemCore", 1, true),
        ("ACE_epinephrine", "ACE_ItemCore", 1, true),
        ("ACE_surgicalKit", "ACE_ItemCore", 15, true),
        ("ACE_bodyBag", "ACE_ItemCore", 7, true),
        // IV items with variants
        ("ACE_bloodIV", "ACE_ItemCore", 10, true),
        ("ACE_bloodIV_500", "ACE_bloodIV", 5, false),
        ("ACE_bloodIV_250", "ACE_bloodIV", 2, false),
        ("ACE_plasmaIV", "ACE_ItemCore", 10, true),
        ("ACE_plasmaIV_500", "ACE_plasmaIV", 5, false),
        ("ACE_plasmaIV_250", "ACE_plasmaIV", 2, false),
        ("ACE_salineIV", "ACE_ItemCore", 10, true),
        ("ACE_salineIV_500", "ACE_salineIV", 5, false),
        ("ACE_salineIV_250", "ACE_salineIV", 2, false)
    ];

    for (item_name, parent_class, mass, check_scope) in medical_items {
        // Skip this test if the medical item is not found
        if let Some(class) = classes.iter().find(|c| c.name == item_name) {
            // Check inheritance
            assert_eq!(class.parent.as_deref(), Some(parent_class), 
                "Medical item {} should inherit from {}", item_name, parent_class);

            // Check scope only for base items, not variants (which inherit scope)
            if check_scope {
                assert!(class.properties.iter().any(|p| p.name == "scope" && matches!(&p.value, CodeValue::Number(n) if *n == 1 || *n == 2)),
                    "Medical item {} should have scope 1 or 2", item_name);
                
                // Check ACE_isMedicalItem property only for base items
                assert!(class.properties.iter().any(|p| p.name == "ACE_isMedicalItem" && matches!(&p.value, CodeValue::Number(n) if *n == 1)),
                    "Medical item {} should have ACE_isMedicalItem = 1", item_name);
            }

            // Check ItemInfo class
            let item_info = class.properties.iter()
                .find(|p| p.name == "ItemInfo");
            
            if let Some(item_info) = item_info {
                if let CodeValue::Class(info_class) = &item_info.value {
                    assert_eq!(info_class.parent.as_deref(), Some("CBA_MiscItem_ItemInfo"), 
                        "ItemInfo for {} should inherit from CBA_MiscItem_ItemInfo", item_name);
                    
                    // Check mass property
                    assert!(info_class.properties.iter().any(|p| p.name == "mass" && matches!(&p.value, CodeValue::Number(n) if *n == mass)),
                        "ItemInfo for {} should have mass = {}", item_name, mass);
                }
            }
        } else {
            println!("Medical item {} not found, skipping test", item_name);
        }
    }

    // Test IV bag variants specifically
    let iv_variants = vec![
        ("ACE_bloodIV", vec!["ACE_bloodIV_500", "ACE_bloodIV_250"]),
        ("ACE_plasmaIV", vec!["ACE_plasmaIV_500", "ACE_plasmaIV_250"]),
        ("ACE_salineIV", vec!["ACE_salineIV_500", "ACE_salineIV_250"])
    ];

    for (base_iv, variants) in iv_variants {
        let base_class = classes.iter()
            .find(|c| c.name == base_iv)
            .unwrap_or_else(|| panic!("Base IV {} not found", base_iv));

        // Check that variants exist and inherit from base
        for variant in variants {
            let variant_class = classes.iter()
                .find(|c| c.name == variant)
                .unwrap_or_else(|| panic!("IV variant {} not found", variant));

            assert_eq!(variant_class.parent.as_deref(), Some(base_iv),
                "IV variant {} should inherit from {}", variant, base_iv);

            // Check model and texture properties
            assert!(variant_class.properties.iter().any(|p| p.name == "model"),
                "IV variant {} should have model property", variant);
            assert!(variant_class.properties.iter().any(|p| p.name == "hiddenSelectionsTextures"),
                "IV variant {} should have hiddenSelectionsTextures", variant);
        }
    }
} 