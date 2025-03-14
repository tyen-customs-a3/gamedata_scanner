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
        let class = classes.iter()
            .find(|c| c.name == base_class)
            .unwrap_or_else(|| panic!("Base class {} not found", base_class));
        
        // Forward declarations should have no properties
        assert!(class.properties.is_empty(), "Base class {} should have no properties", base_class);
    }

    // Test uniform items and their inheritance
    let uniform_items = vec![
        ("bw_uniform_combat_fleck", "Uniform_Base"),
        ("bw_uniform_combat_rs_fleck", "Uniform_Base")
    ];

    for (item, parent) in uniform_items {
        let class = classes.iter()
            .find(|c| c.name == item)
            .unwrap_or_else(|| panic!("Uniform item {} not found", item));

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
    }
} 