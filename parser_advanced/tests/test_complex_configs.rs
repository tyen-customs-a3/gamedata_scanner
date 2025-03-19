#[cfg(test)]
mod tests {
    use parser_advanced::CodeParser;
    use gamedata_scanner_models::PropertyValue;
    use std::path::Path;

    #[test]
    fn test_dialog_configs() {
        let content = r#"
            class CfgUIControls {
                class ArmaUIClass {
                    idc = 1234;
                    text = "Hello World";
                    style = 2;
                    position[] = {0.5, 0.5, 0.3, 0.2};
                };
            };
            
            class RscControlsGroup {
                class HScrollbar;
                class VScrollbar;
                
                idc = -1;
                x = 0;
                y = 0;
                w = 1;
                h = 1;
                shadow = 0;
                shadowColor[] = {0, 0, 0, 0.5};
                color[] = {1, 1, 1, 1};
                colorText[] = {1, 1, 1, 1};
                colorBackground[] = {0, 0, 0, 0};
                colorBackgroundActive[] = {0, 0, 0, 0};
                colorDisabled[] = {1, 1, 1, 0.25};
                colorBackgroundDisabled[] = {0, 0, 0, 0};
            };
            
            class ace_StaminaBarContainer: RscControlsGroup {
                idc = 193;
                x = 0.5;
                y = 0.5;
                w = 10 * GUI_GRID_W;
                h = 1 * GUI_GRID_H;
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let file_path = Path::new("tests/fixtures/dialog_configs.hpp");
        let classes = parser.parse_classes(file_path);
        
        assert!(classes.iter().any(|c| c.name == "ace_StaminaBarContainer"));
    }

    #[test]
    fn test_cfgsounds_with_paths() {
        let content = r#"
            class CfgSounds {
                sounds[] = {};
                
                class ace_explosion_sound {
                    name = "ace_explosion_sound";
                    sound[] = {"path\to\sound.ogg", 1, 1, 200};
                    titles[] = {};
                };
                
                class ace_explosion_ringing {
                    name = "ace_explosion_ringing";
                    sound[] = {"path\to\sound2.ogg", 1, 1, 200};
                    titles[] = {};
                };
                
                class ace_medical_heartbeat {
                    name = "ace_medical_heartbeat";
                    sound[] = {"path\to\heartbeat.ogg", 2, 1, 10};
                    titles[] = {};
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let file_path = Path::new("tests/fixtures/cfgsounds.hpp");
        let classes = parser.parse_classes(file_path);
        
        assert!(classes.iter().any(|c| c.name == "CfgSounds"));
    }

    #[test]
    fn test_ace_settings() {
        let content = r#"
            // Emulate ACE settings config
            #define ESTRING(var1,var2) QUOTE(TRIPLES(STR,DOUBLES(PREFIX,var1),var2))
            #define QUOTE(var1) #var1
            #define TRIPLES(var1,var2,var3) ##var1##_##var2##_##var3
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace
            
            class ACE_Settings {
                class GVAR(enabled) {
                    category = ECSTRING(SettingsMenu,Category_Medical);
                    displayName = CSTRING(MedicalSettings_basicMedicalSimulation_DisplayName);
                    description = CSTRING(MedicalSettings_basicMedicalSimulation_Description);
                    value = 1;
                    typeName = "BOOL";
                };
                class GVAR(playerDamageThreshold) {
                    category = ECSTRING(SettingsMenu,Category_Medical);
                    displayName = CSTRING(MedicalSettings_playerDamageThreshold_DisplayName);
                    description = CSTRING(MedicalSettings_playerDamageThreshold_Description);
                    value = 1;
                    typeName = "SCALAR";
                    values[] = {"Disabled", 0.1, 0.2, 0.5, 0.8, 1, 1.5, 2, 3};
                };
                class GVAR(spontaneousWakeUpChance) {
                    category = ECSTRING(SettingsMenu,Category_Medical);
                    displayName = CSTRING(MedicalSettings_spontaneousWakeUpChance_DisplayName);
                    description = CSTRING(MedicalSettings_spontaneousWakeUpChance_Description);
                    value = 0.05;
                    typeName = "SCALAR";
                    minValue = 0;
                    maxValue = 1;
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let file_path = Path::new("tests/fixtures/ace_settings.hpp");
        let classes = parser.parse_classes(file_path);
        
        assert!(classes.iter().any(|c| c.name == "ACE_Settings"));
    }

    #[test]
    fn test_nested_config_with_inheritance() {
        let content = r#"
            class CfgVehicles {
                class Land;
                class LandVehicle: Land {
                    displayName = "Land Vehicle";
                };
                class Car: LandVehicle {
                    displayName = "Car";
                };
                class Tank: LandVehicle {
                    displayName = "Tank";
                };
                
                class Helicopter;
                class Air {
                    displayName = "Air Vehicle";
                    
                    class NewSubClass {
                        displayName = "Sub Air Class";
                    };
                };
                class Plane: Air {
                    displayName = "Plane";
                };
                class Helicopter_Base_F: Helicopter {
                    class NewSubClass: NewSubClass {
                        scope = 2;
                    };
                };
            };
            
            class CfgWeapons {
                class ItemCore;
                class Rifle_Base_F {
                    class WeaponSlotsInfo {
                        allowedSlots[] = {801, 901, 701, 702}; 
                    };
                };
                class InventoryFlashLightItem_Base_F {
                    class ItemInfo;
                };
                
                class ACE_Flashlight_Base: ItemCore {
                    class ItemInfo: InventoryFlashLightItem_Base_F {
                        mass = 4;
                    };
                };
                
                class ACE_Flashlight_MX991: ACE_Flashlight_Base {
                    displayName = "Fulton MX-991";
                    descriptionShort = "Tactical Light";
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let file_path = Path::new("tests/fixtures/nested_inheritance.hpp");
        let classes = parser.parse_classes(file_path);
        
        println!("Config properties count: {}", classes.len());
        for class in &classes {
            println!("Class: {} (parent: {:?})", class.name, class.parent);
        }
        
        // Basic tests
        assert!(classes.iter().any(|c| c.name == "CfgVehicles"));
        assert!(classes.iter().any(|c| c.name == "CfgWeapons"));
        
        // Test inheritance
        let land_vehicle = classes.iter().find(|c| c.name == "LandVehicle").unwrap();
        assert_eq!(land_vehicle.parent.as_deref(), Some("Land"));
        
        let car = classes.iter().find(|c| c.name == "Car").unwrap();
        assert_eq!(car.parent.as_deref(), Some("LandVehicle"));
        
        // Test nested classes
        let ace_flashlight = classes.iter().find(|c| c.name == "ACE_Flashlight_Base").unwrap();
        assert!(ace_flashlight.properties.iter().any(|p| 
            p.name == "ItemInfo" && 
            matches!(&p.value, PropertyValue::Class(c) if c.parent.as_deref() == Some("InventoryFlashLightItem_Base_F"))
        ));
    }

    #[test]
    fn test_complex_nested_properties() {
        let content = r#"
            class CfgWeapons {
                class LauncherCore {
                    scope = 0;
                };
                class Launcher: LauncherCore {
                    scope = 0;
                    
                    // Basic array properties
                    magazines[] = {"magazine1", "magazine2"};
                    
                    // Nested array properties
                    muzzles[] = {"this", "muzzle2"};
                    
                    // Empty arrays
                    items[] = {};
                    linkeditems[] = {};
                    
                    // Properties with inheritance
                    class GunParticles {
                        class FirstEffect {
                            effectName = "RocketBackEffectsRPGNT";
                            positionName = "Konec hlavne";
                            directionName = "Usti hlavne";
                        };
                    };
                    
                    // Complex nested properties
                    class WeaponSlotsInfo {
                        mass = 70;
                        allowedSlots[] = {901};
                        
                        class MuzzleSlot {
                            linkProxy = "\A3\data_f\proxies\weapon_slots\MUZZLE";
                            displayName = "$STR_A3_CFGWEAPONS_ABSL_MUZZLE0";
                            
                            class compatibleItems {
                                muzzle_snds_B = 1;
                                muzzle_snds_B_arid_F = 1;
                                muzzle_snds_B_khk_F = 1;
                                muzzle_snds_B_lush_F = 1;
                                muzzle_snds_B_snd_F = 1;
                            };
                        };
                    };
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let file_path = Path::new("tests/fixtures/complex_nested.hpp");
        let classes = parser.parse_classes(file_path);
        
        assert!(classes.iter().any(|c| c.name == "CfgWeapons"));
        
        // Test for the Launcher class
        if let Some(launcher) = classes.iter().find(|c| c.name == "Launcher") {
            // Check parent
            assert_eq!(launcher.parent.as_deref(), Some("LauncherCore"));
            
            // Check for WeaponSlotsInfo and nested properties
            assert!(launcher.properties.iter().any(|p| 
                p.name == "WeaponSlotsInfo" && 
                matches!(&p.value, PropertyValue::Class(c) if c.properties.iter().any(|p| p.name == "mass"))
            ));
            
            // Check for magazine array
            assert!(launcher.properties.iter().any(|p| 
                p.name == "magazines" && 
                matches!(&p.value, PropertyValue::Array(a) if a.len() == 2)
            ));
        } else {
            panic!("Launcher class not found");
        }
    }
} 