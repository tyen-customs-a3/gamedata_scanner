#[cfg(test)]
mod tests {
    use parser_code::{CodeParser, CodeValue};

    #[test]
    fn test_prep_macros() {
        let content = r#"
            #define PREP(var1) DOUBLES(TRIPLES(PREFIX,fnc,var1),_fnc)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define TRIPLES(var1,var2,var3) ##var1##_##var2##_##var3
            #define PREFIX ace

            PREP(addDutyFactor);
            PREP(calculateAmmoTemperatureVelocityShift);
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        // This should at least not crash the parser
        let _ = parser.parse_classes();
    }

    #[test]
    fn test_config_declarations() {
        let content = r#"
            class CfgVehicles {
                class Man;
                class CAManBase: Man {
                    class ACE_SelfActions {
                        class ACE_Equipment {
                            class GVAR(throwGrenadeAction) {
                                displayName = CSTRING(ThrowGrenade);
                                condition = QUOTE(_player call FUNC(hasGrenades));
                                statement = "";
                            };
                        };
                    };
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "CfgVehicles"));
    }

    #[test]
    fn test_nested_config_declarations() {
        let content = r#"
            class ACE_Settings {
                class GVAR(enabled) {
                    category = QUOTE(GVAR(Advanced));
                    displayName = CSTRING(Enabled);
                    description = CSTRING(Enabled_Description);
                    typeName = "BOOL";
                    value = 1;
                };
                class GVAR(enableAimCoef) {
                    category = QUOTE(GVAR(Advanced));
                    displayName = CSTRING(EnableAimCoef);
                    description = CSTRING(EnableAimCoef_Description);
                    typeName = "BOOL";
                    value = 1;
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert!(classes.iter().any(|c| c.name == "ACE_Settings"));
    }

    #[test]
    fn test_complex_class_parameters() {
        let content = r#"
            class GVAR(StaminaBarContainer): RscControlsGroupNoScrollbars {
                idd = -1;
                onLoad = QUOTE(uiNamespace setVariable [ARR_2(QQGVAR(staminaBar),_this select 0)]);
                onUnload = QUOTE(uiNamespace setVariable [ARR_2(QQGVAR(staminaBar),nil)]);
                
                class controls {
                    class StaminaBar: RscProgress {
                        idc = 1;
                        x = "0";
                        y = "0";
                        w = "safezoneW * 0.15";
                        h = "safezoneH * 0.02";
                    };
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        // This test is primarily to ensure it doesn't crash
        assert!(classes.len() > 0);
    }

    #[test]
    fn test_compound_macros() {
        let content = r#"
            #define ARR_2(ARG1,ARG2) [ARG1,ARG2]
            #define QQGVAR(var1) QUOTE(QGVAR(var1))
            #define QGVAR(var1) QUOTE(GVAR(var1))
            #define GVAR(var1) DOUBLES(PREFIX,var1)
            #define DOUBLES(var1,var2) ##var1##_##var2
            #define QUOTE(var1) #var1
            #define PREFIX ace

            class SomeClass {
                condition = QUOTE(_this call FUNC(someFunction));
                statement = QUOTE([ARR_2(_player,_target)] call FUNC(doSomething));
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        let classes = parser.parse_classes();
        
        assert_eq!(classes.len(), 1);
    }

    #[test]
    fn test_script_component_patterns() {
        let content = r#"
            #include "script_component.hpp"
            
            #ifdef FAST_PROGRESSBARS
                #define PROGRESS_INTERVALS 0.01
            #else
                #define PROGRESS_INTERVALS 0.05
            #endif
            
            PREP(myFunction);
            
            #define EVENT_PLAYER_FIRED player, "FiredMan", FUNC(handlePlayerFired)
            
            class CfgSounds {
                class GVAR(click) {
                    name = QGVAR(click);
                    sound[] = {QPATHTOF(sounds\click.ogg), 1, 1, 15};
                    titles[] = {};
                };
            };
        "#;
        
        let parser = CodeParser::new(content).unwrap();
        // This should at least not crash the parser
        let _ = parser.parse_classes();
    }
} 