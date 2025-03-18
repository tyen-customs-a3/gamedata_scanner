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

    // Test uniform items and their inheritance
    let uniform_items = vec![
        ("bw_uniform_combat_fleck", "Uniform_Base"),
        ("bw_uniform_combat_rs_fleck", "Uniform_Base")
    ];

    for (item, parent) in uniform_items {
        // Fail the test if the uniform item is not found
        let class = classes.iter().find(|c| c.name == item)
            .unwrap_or_else(|| panic!("Uniform item {} not found", item));
            
        assert_eq!(class.parent.as_deref(), Some(parent), "Uniform {} should inherit from {}", item, parent);
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
    
    // Create a temporary directory for the test
    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("ace_cfg.hpp");
    std::fs::write(&test_file, &content).unwrap();

    // Set up workspace
    let workspace = hemtt_workspace::Workspace::builder()
        .physical(&temp_dir.path().to_path_buf(), hemtt_workspace::LayerType::Source)
        .finish(None, false, &hemtt_common::config::PDriveOption::Disallow)
        .unwrap();

    // Process and parse the config
    let source = workspace.join("ace_cfg.hpp").unwrap();
    let processed = hemtt_preprocessor::Processor::run(&source).unwrap();
    println!("Processed output:\n{}", processed.as_str());
    
    let parsed = hemtt_config::parse(None, &processed);
    let workspacefiles = hemtt_workspace::reporting::WorkspaceFiles::new();

    // Get diagnostic output and handle errors before proceeding
    let config = match &parsed {
        Ok(config) => {
            // Even successful parse might have warnings/notes
            let messages = config
                .codes()
                .iter()
                .map(|e| e.diagnostic().unwrap().to_string(&workspacefiles))
                .collect::<Vec<_>>();
            
            if !messages.is_empty() {
                println!("\nWarnings/Notes from successful parse:");
                for msg in &messages {
                    println!("{}", msg);
                }
            }
            
            Some(config)
        }
        Err(errors) => {
            println!("\nParse errors encountered:");
            for error in errors {
                if let Some(diag) = error.diagnostic() {
                    println!("Error: {}", diag.to_string(&workspacefiles));
                } else {
                    println!("Error: {}", error.message());
                }
                
                // Print additional context for specific error codes
                match error.ident() {
                    "CCHU" => println!("CCHU Error: Character encoding or syntax error detected. Check for invalid characters or unterminated strings."),
                    "L-C01" => println!("L-C01 Error: Missing quotes around string value."),
                    "L-C04" => println!("L-C04 Error: Invalid class inheritance or missing parent class."),
                    _ => println!("Error code: {}", error.ident()),
                }
            }
            None
        }
    };

    // If we have parse errors, fail the test with detailed information
    let config = config.unwrap_or_else(|| {
        println!("\nProcessed content that failed to parse:");
        println!("{}", processed.as_str());
        panic!("Failed to parse config file. See above errors for details.");
    });

    // Initialize parser with the processed content
    let parser = match CodeParser::new(processed.as_str()) {
        Ok(p) => p,
        Err(e) => {
            println!("\nParser initialization failed with diagnostics:");
            for error in e {
                if let Some(diag) = error.diagnostic() {
                    println!("{}", diag.to_string(&workspacefiles));
                } else {
                    println!("{}", error.message());
                }
                
                // Print additional context for specific error codes
                if let Some(ident) = error.ident().chars().next().map(|c| c.to_string()) {
                    match ident.as_str() {
                        "C" => println!("Syntax or character encoding error detected."),
                        "L" => println!("Logic or semantic error in the config."),
                        "P" => println!("Preprocessing error detected."),
                        _ => {}
                    }
                }
            }
            panic!("Failed to initialize parser. See above errors for details.");
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
    
    let mut missing_base_classes = Vec::new();
    
    // Search in the cfgweapon class properties for the base classes
    for prop in &cfg_weapons.properties {
        if let CodeValue::Class(class) = &prop.value {
            if base_classes.contains(&class.name.as_str()) {
                missing_base_classes.push(class.name.clone());
            }
        }
    }
    // Print missing base classes
    if !missing_base_classes.is_empty() {
        println!("\nMissing base classes:");
        for class_name in &missing_base_classes {
            println!("  - {}", class_name);
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

    let mut missing_medical_items = Vec::new();
    let mut property_errors = Vec::new();

    for (item_name, parent_class, mass, check_scope) in medical_items {
        // Check if the medical item exists
        if let Some(class) = classes.iter().find(|c| c.name == item_name) {
            // Check inheritance
            if class.parent.as_deref() != Some(parent_class) {
                property_errors.push(format!("{} should inherit from {} but inherits from {:?}", 
                    item_name, parent_class, class.parent));
            }

            // Check scope only for base items, not variants (which inherit scope)
            if check_scope {
                if !class.properties.iter().any(|p| p.name == "scope" && matches!(&p.value, CodeValue::Number(n) if *n == 1 || *n == 2)) {
                    property_errors.push(format!("{} is missing scope property or it's not 1 or 2", item_name));
                }
                
                // Check ACE_isMedicalItem property only for base items
                if !class.properties.iter().any(|p| p.name == "ACE_isMedicalItem" && matches!(&p.value, CodeValue::Number(n) if *n == 1)) {
                    property_errors.push(format!("{} is missing ACE_isMedicalItem=1 property", item_name));
                }
            }

            // Check ItemInfo class
            let item_info = class.properties.iter()
                .find(|p| p.name == "ItemInfo");
            
            if let Some(item_info) = item_info {
                if let CodeValue::Class(info_class) = &item_info.value {
                    if info_class.parent.as_deref() != Some("CBA_MiscItem_ItemInfo") {
                        property_errors.push(format!("ItemInfo for {} should inherit from CBA_MiscItem_ItemInfo but inherits from {:?}", 
                            item_name, info_class.parent));
                    }
                    
                    // Check mass property
                    if !info_class.properties.iter().any(|p| p.name == "mass" && matches!(&p.value, CodeValue::Number(n) if *n == mass)) {
                        property_errors.push(format!("ItemInfo for {} should have mass={} but has a different value", 
                            item_name, mass));
                    }
                } else {
                    property_errors.push(format!("ItemInfo for {} is not a class", item_name));
                }
            } else {
                property_errors.push(format!("{} is missing ItemInfo class", item_name));
            }
        } else {
            missing_medical_items.push(item_name);
        }
    }

    // Print missing medical items
    if !missing_medical_items.is_empty() {
        println!("\nMissing medical items ({}):", missing_medical_items.len());
        for item_name in &missing_medical_items {
            println!("  - {}", item_name);
        }
    }

    // Print property errors
    if !property_errors.is_empty() {
        println!("\nProperty errors ({}):", property_errors.len());
        for error in &property_errors {
            println!("  - {}", error);
        }
    }

    // Fail the test if any items are missing or have property errors
    assert!(missing_base_classes.is_empty(), "Missing {} base classes", missing_base_classes.len());
    assert!(missing_medical_items.is_empty(), "Missing {} medical items", missing_medical_items.len());
    assert!(property_errors.is_empty(), "Found {} property errors", property_errors.len());
} 