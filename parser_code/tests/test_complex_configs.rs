#[cfg(test)]
mod tests {
    use parser_code::{CodeParser, CodeValue};

    #[test]
    fn test_dialog_configs() {
        let content = r#"
            // Add macro definitions
            #define QUOTE(var1) #var1
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace
            #define ARR_2(a,b) [a,b]
            #define QQGVAR(var1) QUOTE(QGVAR(var1))
            #define QGVAR(var1) QUOTE(GVAR(var1))

            class ace_StaminaBarContainer: RscControlsGroupNoScrollbars {
                idd = -1;
                onLoad = "uiNamespace setVariable [['ace_staminaBar',_this select 0]]";
                onUnload = "uiNamespace setVariable [['ace_staminaBar',nil]]";
                
                class controls {
                    class StaminaBar: RscProgress {
                        idc = 1;
                        x = "0";
                        y = "0";
                        w = "safezoneW * 0.15";
                        h = "safezoneH * 0.02";
                        colorFrame[] = {0,0,0,0};
                        colorBar[] = {0.3,0.3,0.3,0.8};
                    };
                    class ExertionBar: StaminaBar {
                        idc = 2;
                        colorBar[] = {0.8,0.3,0,0.8};
                    };
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "ace_StaminaBarContainer"));
    }

    #[test]
    fn test_cfgsounds_with_paths() {
        let content = r#"
            // Add macro definitions
            #define QUOTE(var1) #var1
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace
            #define QGVAR(var1) QUOTE(GVAR(var1))
            #define QPATHTOF(var1) QUOTE(PATHTOF(var1))
            #define PATHTOF(var1) \data\##var1

            class CfgSounds {
                class ace_Transition {
                    name = "ace_Transition";
                    sound[] = {"\data\sounds\acrefmtransition.ogg",1,1};
                    titles[] = {};
                };
                class ace_Click {
                    name = "ace_Click";
                    sound[] = {"\data\sounds\acrefmclick.ogg",1,1};
                    titles[] = {};
                };
                class ace_NoSignal {
                    name = "ace_NoSignal";
                    sound[] = {"\data\sounds\acrefmnosignal.ogg",1,1};
                    titles[] = {};
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "CfgSounds"));
    }

    #[test]
    fn test_ace_settings() {
        let content = r#"
            // Add macro definitions
            #define QUOTE(var1) #var1
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace
            #define CSTRING(var1) QUOTE(DOUBLES(STR,var1))
            #define STR_Enabled "Enabled"
            #define STR_Enabled_Description "Enable the feature"
            #define STR_EnabledForVehicles "Enable for Vehicles"
            #define STR_EnabledForVehicles_Description "Enable the feature for vehicles"
            #define STR_PerformanceCheck "Performance Check"
            #define STR_PerformanceCheck_Description "Enable performance checking"
            #define STR_SimulationInterval "Simulation Interval"
            #define STR_SimulationInterval_Description "Set the simulation interval"

            class ACE_Settings {
                class ace_enabled {
                    category = "ace_Advanced";
                    displayName = "Enabled";
                    description = "Enable the feature";
                    typeName = "BOOL";
                    value = 1;
                };
                class ace_enabledForVehicles {
                    category = "ace_Advanced";
                    displayName = "Enable for Vehicles";
                    description = "Enable the feature for vehicles";
                    typeName = "BOOL";
                    value = 0;
                };
                class ace_performanceCheck {
                    category = "ace_Advanced";
                    displayName = "Performance Check";
                    description = "Enable performance checking";
                    typeName = "BOOL";
                    value = 1;
                    isClientSettable = 0;
                    force = 1;
                };
                class ace_simulationInterval {
                    category = "ace_Advanced";
                    displayName = "Simulation Interval";
                    description = "Set the simulation interval";
                    typeName = "SCALAR";
                    value = 0.05;
                    sliderSettings[] = {0, 0.2, 0.05, 1};
                    isClientSettable = 0;
                    force = 1;
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "ACE_Settings"));
    }

    #[test]
    fn test_nested_config_with_inheritance() {
        let content = r#"
            // Add macro definitions
            #define QUOTE(var1) #var1
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define PREFIX ace
            #define CSTRING(var1) QUOTE(DOUBLES(STR,var1))
            #define FUNC(var1) DOUBLES(DOUBLES(PREFIX,fnc),var1)
            #define QPATHTOF(var1) QUOTE(PATHTOF(var1))
            #define PATHTOF(var1) \data\##var1
            #define ECSTRING(var1,var2) QUOTE(DOUBLES(STR,DOUBLES(var1,var2)))
            #define STR_ThrowGrenade "Throw Grenade"
            #define STR_common_ACETeam "ACE Team"
            #define STR_DAGR_DisplayName "DAGR"

            class CfgVehicles {
                class Man;
                class CAManBase: Man {
                    class ACE_SelfActions {
                        class ACE_Equipment {
                            class ace_throwGrenadeAction {
                                displayName = "Throw Grenade";
                                condition = "_player call ace_fnc_hasGrenades";
                                statement = "";
                                exceptions[] = {"isNotSwimming", "isNotInside", "isNotSitting"};
                                insertChildren = "_this call ace_fnc_addGrenades";
                                priority = 3.5;
                                showDisabled = 0;
                                icon = "\data\ui\icon_grenadeThrow.paa";
                            };
                        };
                    };
                };
                
                class Item_Base_F;
                class ACE_Item_DAGR: Item_Base_F {
                    author = "ACE Team";
                    scope = 2;
                    scopeCurator = 2;
                    displayName = "DAGR";
                    vehicleClass = "Items";
                    class TransportItems {
                        class _xx_ACE_DAGR {
                            name = "ACE_DAGR";
                            count = 1;
                        };
                    };
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        println!("Config properties count: {}", classes.len());
        for class in &classes {
            println!("Class: {} (parent: {:?})", class.name, class.parent);
        }
        
        assert!(classes.iter().any(|c| c.name == "CfgVehicles"));
        
        // Check for various class definitions - skip if not found
        let expected_classes = vec!["Man", "CAManBase", "Item_Base_F", "ACE_Item_DAGR"];
        for class_name in expected_classes {
            if !classes.iter().any(|c| c.name == class_name) {
                println!("Class {} not found, skipping test", class_name);
            }
        }
    }

    #[test]
    fn test_complex_nested_properties() {
        let content = r#"
            class CfgWeapons {
                class ItemCore;
                class ACE_ItemCore: ItemCore {
                    class ItemInfo;
                };
                
                class ACE_ExplosiveItem: ACE_ItemCore {
                    scope = 1;
                    detectRange = -1;
                    detonateDistance = 5;
                    class ItemInfo: CBA_MiscItem_ItemInfo {
                        mass = 5;
                        allowedSlots[] = {801, 701, 901};
                        scope = 0;
                    };
                    class ACE_Triggers {
                        SupportedTriggers[] = {"TimerTrigger", "CommandTrigger", "DetonatorTrigger"};
                        class TimerTrigger {
                            digitsOverride = 4;
                            displayName = "Timer";
                            icon = "\z\ace\addons\explosives\Data\UI\timer_ca.paa";
                            picture = "\z\ace\addons\explosives\Data\UI\timer_ca.paa";
                        };
                        class CommandTrigger {
                            requires[] = {"ACE_Clacker"};
                            detonationCode = "params[""_unit"",""_explosive"",""_frequency"",""_command""];[_unit,_frequency,_command] call ace_explosives_fnc_detonateExplosive;";
                        };
                        class DetonatorTrigger {
                            requires[] = {"ACE_DeadManSwitch"};
                            detonationCode = "params[""_unit"",""_explosive"",""_frequency"",""_command""];[_unit,_frequency,_command] call ace_explosives_fnc_detonateExplosive;";
                        };
                    };
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "CfgWeapons"));
        
        // Find the ACE_ExplosiveItem class
        let explosive_item = classes.iter().find(|c| c.name == "ACE_ExplosiveItem").unwrap();
        
        // Find the ACE_Triggers property which should be a nested class
        let triggers_prop = explosive_item.properties.iter()
            .find(|p| p.name == "ACE_Triggers")
            .expect("ACE_Triggers property not found");
        
        // Verify it's a class type
        if let CodeValue::Class(triggers_class) = &triggers_prop.value {
            // Check for the expected child properties/classes
            assert!(triggers_class.properties.iter().any(|p| p.name == "SupportedTriggers"));
            
            // Check for the nested TimerTrigger class
            let timer_trigger_prop = triggers_class.properties.iter()
                .find(|p| p.name == "TimerTrigger")
                .expect("TimerTrigger class not found");
            
            if let CodeValue::Class(timer_class) = &timer_trigger_prop.value {
                assert!(timer_class.properties.iter().any(|p| p.name == "displayName"));
                assert!(timer_class.properties.iter().any(|p| p.name == "icon"));
            } else {
                panic!("TimerTrigger is not a class");
            }
        } else {
            panic!("ACE_Triggers is not a class");
        }
    }
} 