use parser_advanced::CodeParser;
use models::PropertyValue;
use std::fs;
use std::path::Path;

#[test]
fn test_loadout_parsing() {
    let content = fs::read_to_string("tests/fixtures/test_config_small.cpp").unwrap();
    let parser = CodeParser::new(&content).unwrap();
    let file_path = Path::new("tests/fixtures/test_config_small.cpp");
    let classes = parser.parse_classes(file_path);

    // Debug print all classes
    println!("Found classes:");
    for class in &classes {
        println!("Class: {} (parent: {:?})", class.name, class.parent);
    }

    // Verify we parsed some key classes
    assert!(classes.iter().any(|c| c.name == "bw_uniform_combat_fleck"));
    assert!(classes.iter().any(|c| c.name == "bw_uniform_combat_rs_fleck"));
    assert!(classes.iter().any(|c| c.name == "CfgWeapons"));
    
    // Check for uniform items
    let uniform_items = vec![
        ("bw_uniform_combat_fleck", "Uniform_Base"),
        ("bw_uniform_combat_rs_fleck", "Uniform_Base")
    ];

    for (item, parent) in uniform_items {
        // Find the uniform item
        let class = classes.iter().find(|c| c.name == item)
            .unwrap_or_else(|| panic!("Uniform item {} not found", item));
            
        assert_eq!(class.parent.as_deref(), Some(parent), "Uniform {} should inherit from {}", item, parent);
    }
}

#[test]
fn test_ace_medical_items() {
    // This is a common pattern in ACE configs
    let content = r#"
        class CfgWeapons {
            class ACE_ItemCore;
            class CBA_MiscItem_ItemInfo;
            
            class ACE_fieldDressing: ACE_ItemCore {
                scope = 2;
                author = "$STR_ACE_Common_ACETeam";
                picture = "\ACE_Medical_Items\ui\fieldDressing_ca.paa";
                displayName = "$STR_ACE_Medical_Items_Bandage_Basic_Display";
                descriptionShort = "$STR_ACE_Medical_Items_Bandage_Basic_Desc";
                descriptionUse = "$STR_ACE_Medical_Items_Bandage_Basic_Use";
                model = "\A3\Structures_F_EPA\Items\Medical\Bandage_F.p3d";
                class ItemInfo: CBA_MiscItem_ItemInfo {
                    mass = 1;
                };
            };
            
            class ACE_elasticBandage: ACE_ItemCore {
                scope = 2;
                author = "$STR_ACE_Common_ACETeam";
                picture = "\ACE_Medical_Items\ui\elasticBandage_ca.paa";
                displayName = "$STR_ACE_Medical_Items_Bandage_Elastic_Display";
                descriptionShort = "$STR_ACE_Medical_Items_Bandage_Elastic_Desc";
                descriptionUse = "$STR_ACE_Medical_Items_Bandage_Elastic_Use";
                model = "\A3\Structures_F_EPA\Items\Medical\Bandage_F.p3d";
                class ItemInfo: CBA_MiscItem_ItemInfo {
                    mass = 1;
                };
            };
        };
    "#;
    
    let parser = CodeParser::new(content).unwrap();
    let file_path = Path::new("tests/fixtures/ace_medical_items.hpp");
    let classes = parser.parse_classes(file_path);
    
    // Debug: Print all top-level classes found
    println!("\nFound {} top-level classes:", classes.len());
    for class in &classes {
        println!("Class: {} (parent: {:?})", class.name, class.parent);
    }
    
    // Verify we have medical items
    assert!(classes.iter().any(|c| c.name == "ACE_fieldDressing"));
    assert!(classes.iter().any(|c| c.name == "ACE_elasticBandage"));
    
    // Verify inheritance
    let field_dressing = classes.iter().find(|c| c.name == "ACE_fieldDressing").unwrap();
    assert_eq!(field_dressing.parent.as_deref(), Some("ACE_ItemCore"));
    
    // Check item info
    assert!(field_dressing.properties.iter().any(|p| 
        p.name == "ItemInfo" && 
        matches!(&p.value, PropertyValue::Class(c) if c.parent.as_deref() == Some("CBA_MiscItem_ItemInfo"))
    ));
} 